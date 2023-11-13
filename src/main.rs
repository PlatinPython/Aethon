#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod screens;

use std::path::PathBuf;
use std::{env, io};

use iced::futures::lock::Mutex;
use iced::{executor, Application, Command, Element, Settings, Theme};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use single_instance::SingleInstance;
use tokio::fs;

use crate::screens::error::Error;
use crate::screens::startup::{load, Startup};
use crate::screens::{startup, Messages, Screen, Screens};

const UUID: &str = "aethon-f082c8ab-df27-4daf-9d09-48ff15ef0204";

static INSTANCE: Lazy<SingleInstance> =
    Lazy::new(|| SingleInstance::new(UUID).expect("SingleInstance object creation failed."));

static CONFIG: Lazy<Mutex<Config>> = Lazy::new(|| Mutex::new(Config::default()));

#[derive(Debug)]
struct Manager {
    current_screen: Screens,
}

#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
struct Config {
    launcher_path: Option<PathBuf>,
}

impl Config {
    async fn save(&self) -> Result<(), Errors> {
        let config_path = env::current_exe()
            .map_err(|error| Errors::Io(error.kind()))?
            .parent()
            .ok_or(Errors::NoParent)?
            .join("config.json");
        fs::write(
            config_path,
            serde_json::to_string(self).map_err(|error| Errors::Json(error.to_string()))?,
        )
        .await
        .map_err(|error| Errors::Io(error.kind()))
    }
}

#[derive(Debug, Clone)]
enum Errors {
    Io(io::ErrorKind),
    Json(String),
    NoParent,
}

impl Application for Manager {
    type Executor = executor::Default;
    type Message = Messages;
    type Theme = Theme;
    type Flags = ();

    fn new(_: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Manager {
                current_screen: Startup.into(),
            },
            Command::perform(load(), |screens| {
                Messages::Startup(startup::Message::Loaded(screens))
            }),
        )
    }

    fn title(&self) -> String {
        String::from("Aethon")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        if let Messages::Save(Err(error)) = message {
            self.update_screen(Screens::Error(Error::new(
                error,
                Box::new(self.current_screen.clone()),
            )));
            return Command::none();
        }
        match &mut self.current_screen {
            Screens::Startup(screen) => {
                if let Messages::Startup(message) = message {
                    let (command, screen) = screen.update(message);
                    if let Some(screen) = screen {
                        self.update_screen(screen)
                    }
                    command
                } else {
                    Command::none()
                }
            }
            Screens::FolderNotEmptyWarn(screen) => {
                if let Messages::FolderNotEmptyWarn(message) = message {
                    let (command, screen) = screen.update(message);
                    if let Some(screen) = screen {
                        self.update_screen(screen)
                    }
                    command
                } else {
                    Command::none()
                }
            }
            Screens::SingleInstanceWarn(screen) => {
                if let Messages::SingleInstanceWarn(message) = message {
                    let (command, screen) = screen.update(message);
                    if let Some(screen) = screen {
                        self.update_screen(screen)
                    }
                    command
                } else {
                    Command::none()
                }
            }
            Screens::Setup(screen) => {
                if let Messages::Setup(message) = message {
                    let (command, screen) = screen.update(message);
                    if let Some(screen) = screen {
                        self.update_screen(screen)
                    }
                    command
                } else {
                    Command::none()
                }
            }
            Screens::Main(screen) => {
                if let Messages::Main(message) = message {
                    let (command, screen) = screen.update(message);
                    if let Some(screen) = screen {
                        self.update_screen(screen)
                    }
                    command
                } else {
                    Command::none()
                }
            }
            Screens::Error(screen) => {
                if let Messages::Error(message) = message {
                    let (command, screen) = screen.update(message);
                    if let Some(screen) = screen {
                        self.update_screen(screen);
                    }
                    command
                } else {
                    Command::none()
                }
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        match &self.current_screen {
            Screens::Startup(screen) => screen.view(),
            Screens::FolderNotEmptyWarn(screen) => screen.view(),
            Screens::SingleInstanceWarn(screen) => screen.view(),
            Screens::Setup(screen) => screen.view(),
            Screens::Main(screen) => screen.view(),
            Screens::Error(screen) => screen.view(),
        }
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

impl Manager {
    fn update_screen(&mut self, screen: Screens) {
        self.current_screen = screen;
    }
}

fn main() -> Result<(), iced::Error> {
    Manager::run(Settings::default())
}
