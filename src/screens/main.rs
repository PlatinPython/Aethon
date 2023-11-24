use std::fs;
use std::path::PathBuf;
use std::process::Command as Program;

use iced::widget::{button, row};
use iced::{Alignment, Command, Element};
use serde_json::{json, Value};

use crate::instance::Instance;
use crate::screens::error::Error;
use crate::screens::{centering_container, Messages, Screen, Screens};
use crate::widget::instance_card;
use crate::{paths, Errors};

#[derive(Debug, Clone)]
pub(crate) struct Main {
    launcher_path: PathBuf,
    instances: Vec<Instance>,
    current_hovered: (usize, bool),
}

#[derive(Debug, Clone)]
pub(crate) enum Message {
    AddInstance,
    NewInstance(Result<Instance, Errors>),
    TryRun(usize),
    Run(Result<(), Errors>),
}

impl Main {
    pub(crate) fn new(launcher_path: PathBuf, instances: Vec<Instance>) -> Self {
        Main {
            launcher_path,
            instances,
            current_hovered: (0, false),
        }
    }
}

impl Screen for Main {
    type Message = Message;

    fn update(&mut self, message: Self::Message) -> (Command<Messages>, Option<Screens>) {
        match message {
            Message::AddInstance => (
                Command::perform(Instance::new("Test"), |result| {
                    Messages::Main(Message::NewInstance(result))
                }),
                None,
            ),
            Message::NewInstance(result) => (
                Command::none(),
                match result {
                    Ok(instance) => {
                        self.instances.push(instance);

                        None
                    }
                    Err(error) => Some(Screens::Error(Error::new(
                        error,
                        Box::new(Screens::Main(self.clone())),
                    ))),
                },
            ),
            Message::TryRun(i) => {
                self.current_hovered = (i, true);
                (
                    Command::perform(
                        run(self.launcher_path.clone(), self.instances[i].clone()),
                        |result| Messages::Main(Message::Run(result)),
                    ),
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
            row(self
                .instances
                .iter()
                .enumerate()
                .map(|(i, instance)| {
                    instance_card::InstanceCard::new(
                        Messages::Main(Message::TryRun(i)),
                        if self.current_hovered.0 == i {
                            self.current_hovered.1
                        } else {
                            false
                        },
                        instance,
                    )
                    .width(120)
                    .height(150)
                    .into()
                })
                .collect())
            .push(button("Add").on_press(Messages::Main(Message::AddInstance)))
            .spacing(10)
            .align_items(Alignment::Center),
        ))
        // .explain(iced::Color::WHITE)
    }
}

impl From<Main> for Screens {
    fn from(value: Main) -> Self {
        Screens::Main(value)
    }
}

async fn run(launcher_path: PathBuf, instance: Instance) -> Result<(), Errors> {
    println!("Selected path: {:?}", launcher_path);

    add_profile(instance).await?;

    Program::new(launcher_path)
        .spawn()
        .map_err(|error| Errors::Io(error.kind()))?;
    Ok(())
}

async fn add_profile(instance: Instance) -> Result<(), Errors> {
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
        "gameDir": instance.path(),
    });
    fs::write(
        &path,
        serde_json::to_string(&value).map_err(|error| Errors::Json(error.to_string()))?,
    )
    .map_err(|error| Errors::Io(error.kind()))
}
