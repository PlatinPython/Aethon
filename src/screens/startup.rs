use std::fs;
use std::ops::DerefMut;

use iced::widget::text;
use iced::{Command, Element};

use crate::screens::error::Error;
use crate::screens::folder_warn::FolderNotEmptyWarn;
use crate::screens::instance_warn::SingleInstanceWarn;
use crate::screens::main::Main;
use crate::screens::setup::load_setup;
use crate::screens::{centering_container, Messages, Screen, Screens};
use crate::{paths, Config, Errors, CONFIG, INSTANCE};

#[derive(Debug, Clone)]
pub(crate) struct Startup;

#[derive(Debug, Clone)]
pub(crate) enum Message {
    Loaded(Result<Screens, Errors>),
}

impl Screen for Startup {
    type Message = Message;

    fn update(&mut self, message: Self::Message) -> (Command<Messages>, Option<Screens>) {
        let Message::Loaded(result) = message;
        (
            Command::none(),
            match result {
                Ok(screen) => Some(screen),
                Err(error) => {
                    println!("Hi");
                    Some(Screens::Error(Error::new(
                        error,
                        Box::new(Screens::Startup(self.clone())),
                    )))
                }
            },
        )
    }

    fn view(&self) -> Element<'_, Messages> {
        centering_container(text("Loading...")).into()
    }
}

impl From<Startup> for Screens {
    fn from(value: Startup) -> Self {
        Screens::Startup(value)
    }
}

pub(crate) async fn load() -> Result<Screens, Errors> {
    if !INSTANCE.is_single() {
        return Ok(SingleInstanceWarn.into());
    }
    let config_path = paths::CONFIG.clone()?;
    if !config_path.exists()
        && paths::CURRENT_DIR
            .clone()?
            .read_dir()
            .map_err(|error| Errors::Io(error.kind()))?
            .count()
            > 1
    {
        return Ok(FolderNotEmptyWarn.into());
    }
    if config_path.exists() {
        *CONFIG.lock().await.deref_mut() = serde_json::from_str::<Config>(
            &fs::read_to_string(config_path).map_err(|error| Errors::Io(error.kind()))?,
        )
        .map_err(|error| Errors::Json(error.to_string()))?;
    }
    if let Some(launcher_path) = &CONFIG.lock().await.launcher_path {
        if launcher_path.exists() {
            return Ok(Main::new(launcher_path.clone()).into());
        }
    }
    load_setup().await.map(Into::into)
}
