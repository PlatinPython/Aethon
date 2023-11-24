use std::path::{Path, PathBuf};

use iced::futures::{StreamExt, TryFutureExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use tokio::fs;
use tokio::fs::read_dir;
use tokio_stream::wrappers::ReadDirStream;

use crate::{paths, Errors};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Instance {
    name: String,
    folder: String,
    #[serde(skip)]
    path: PathBuf,
}

impl Instance {
    pub(crate) async fn new(name: &str) -> Result<Self, Errors> {
        let (folder_name, path) = create_folder(name).await?;
        let instance = Self {
            name: name.to_string(),
            folder: folder_name,
            path,
        };

        instance.save().await?;

        Ok(instance)
    }

    pub(crate) async fn save(&self) -> Result<(), Errors> {
        fs::write(
            &self.path.join("instance.json"),
            serde_json::to_string(self).map_err(|error| Errors::Json(error.to_string()))?,
        )
        .await
        .map_err(|error| Errors::Io(error.kind()))
    }

    pub(crate) async fn load(path: impl AsRef<Path>) -> Result<Self, Errors> {
        let mut instance: Instance = serde_json::from_str(
            &fs::read_to_string(&path)
                .await
                .map_err(|error| Errors::Io(error.kind()))?,
        )
        .map_err(|error| Errors::Json(error.to_string()))?;
        instance.path = path
            .as_ref()
            .parent()
            .ok_or(Errors::NoParent)?
            .to_path_buf();
        Ok(instance)
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn path(&self) -> &PathBuf {
        &self.path
    }
}

pub(crate) async fn collect_instances() -> Result<Vec<Instance>, Errors> {
    if !paths::INSTANCES.clone()?.exists() {
        fs::create_dir(paths::INSTANCES.clone()?)
            .map_err(|error| Errors::Io(error.kind()))
            .await?;
    }

    ReadDirStream::new(
        read_dir(paths::INSTANCES.clone()?)
            .map_err(|error| Errors::Io(error.kind()))
            .await?,
    )
    .map(|entry| match entry {
        Ok(entry) => Ok(entry.path().join("instance.json")),
        Err(error) => Err(Errors::Io(error.kind())),
    })
    .filter_map(|path: Result<PathBuf, _>| async {
        if path.as_ref().is_ok_and(|path| path.exists()) || path.as_ref().is_err() {
            Some(path)
        } else {
            None
        }
    })
    .then(|path| async {
        match path {
            Ok(path) => Ok(Instance::load(path).await),
            Err(e) => Err(e),
        }
    })
    .try_fold(vec![], |mut vec, instance| async {
        {
            instance.map(|instance| {
                vec.push(instance);
                vec
            })
        }
    })
    .await
}

async fn create_folder(name: &str) -> Result<(String, PathBuf), Errors> {
    let mut counter = 1;
    let mut folder_name = name.to_string();
    while paths::INSTANCES.clone()?.join(&folder_name).exists() {
        folder_name = format!("{} ({})", name, counter);
        counter += 1;
    }
    let path = paths::INSTANCES.clone()?.join(&folder_name);
    fs::create_dir_all(&path)
        .map_err(|error| Errors::Io(error.kind()))
        .await?;
    Ok((folder_name, path))
}
