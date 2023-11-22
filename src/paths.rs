use std::clone::Clone;
use std::env;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use once_cell::sync::Lazy;

use crate::Errors;

pub(crate) static CURRENT_DIR: Lazy<Result<PathBuf, Errors>> = Lazy::new(|| {
    env::current_exe()
        .map_err(|error| Errors::Io(error.kind()))
        .and_then(|path| path.parent().map(Path::to_path_buf).ok_or(Errors::NoParent))
});

pub(crate) static CONFIG: Lazy<Result<PathBuf, Errors>> =
    Lazy::new(|| CURRENT_DIR.clone().map(|path| path.join("config.json")));

pub(crate) static INSTANCE: Lazy<Result<PathBuf, Errors>> =
    Lazy::new(|| CURRENT_DIR.clone().map(|path| path.join("instance")));

pub(crate) static PROFILE: Lazy<Result<PathBuf, Errors>> = Lazy::new(|| {
    dirs::config_dir()
        .map(|path| path.join(".minecraft/launcher_profiles.json"))
        .ok_or(Errors::Io(ErrorKind::NotFound))
});
