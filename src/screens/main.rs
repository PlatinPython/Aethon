use std::fs;
use std::path::PathBuf;
use std::process::Command as Program;

use iced::{Color, Command, Element};
use serde_json::{json, Value};

use crate::screens::error::Error;
use crate::screens::{centering_container, Messages, Screen, Screens};
use crate::widget::instance;
use crate::{paths, Errors};

#[derive(Debug, Clone)]
pub(crate) struct Main(PathBuf, bool);

#[derive(Debug, Clone)]
pub(crate) enum Message {
    TryRun,
    Run(Result<(), Errors>),
}

impl Main {
    pub(crate) fn new(path: PathBuf) -> Self {
        Main(path, false)
    }
}

impl Screen for Main {
    type Message = Message;

    fn update(&mut self, message: Self::Message) -> (Command<Messages>, Option<Screens>) {
        match message {
            Message::TryRun => {
                self.1 = true;
                (
                    Command::perform(run(self.0.clone()), |result| {
                        Messages::Main(Message::Run(result))
                    }),
                    None,
                )
            }
            Message::Run(result) => (
                Command::none(),
                match result {
                    Ok(_) => None,
                    Err(error) => Some(Screens::Error(Error::new(
                        error,
                        Box::new(Screens::Main(self.clone())),
                    ))),
                },
            ),
        }
    }

    fn view(&self) -> Element<'_, Messages> {
        Element::from(centering_container(
            instance::Instance::new(Messages::Main(Message::TryRun), self.1)
                .width(120)
                .height(150),
        ))
        .explain(Color::WHITE)
    }
}

impl From<Main> for Screens {
    fn from(value: Main) -> Self {
        Screens::Main(value)
    }
}

async fn run(launcher_path: PathBuf) -> Result<(), Errors> {
    println!("Selected path: {:?}", launcher_path);

    add_profile().await?;

    Program::new(launcher_path)
        .spawn()
        .map_err(|error| Errors::Io(error.kind()))?;
    Ok(())
}

async fn add_instance() -> Result<PathBuf, Errors> {
    let path = paths::INSTANCE.clone()?;
    if !path.exists() {
        fs::create_dir(&path).map_err(|error| Errors::Io(error.kind()))?;
    }
    Ok(path)
}

async fn add_profile() -> Result<(), Errors> {
    let path = paths::PROFILE.clone()?;
    let mut value = serde_json::from_str::<Value>(
        &fs::read_to_string(&path).map_err(|error| Errors::Io(error.kind()))?,
    )
    .map_err(|error| Errors::Json(error.to_string()))?;
    if !value["profiles"].is_object() {
        value["profiles"] = json!({});
    }
    let profiles = &mut value["profiles"];
    profiles["aethon"] = json!({
        "name": "Aethon",
        "type": "custom",
        "icon": "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAIAAAACAAQMAAAD58POIAAAABlBMVEUAAAD4APit1uGJAAAAI0lEQVRIx2P4DwUMMDAqMCowKjAqQKTAaDCMCowKjAqQKQAABpD8LlM5SL4AAAAASUVORK5CYII",
        "lastVersionId": "latest-release",
        "gameDir": add_instance().await?,
    });
    fs::write(
        &path,
        serde_json::to_string(&value).map_err(|error| Errors::Json(error.to_string()))?,
    )
    .map_err(|error| Errors::Io(error.kind()))
}
