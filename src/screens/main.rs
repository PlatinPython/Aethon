use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command as Program;

use iced::widget::button;
use iced::{Command, Element};
use serde_json::{json, Value};

use crate::screens::{centering_container, Messages, Screen, Screens};

#[derive(Debug, Clone)]
pub(crate) struct Main(PathBuf);

#[derive(Debug, Clone)]
pub(crate) enum Message {
    Run,
}

impl Main {
    pub(crate) fn new(path: PathBuf) -> Self {
        Main(path)
    }
}

impl Screen for Main {
    type Message = Message;

    fn update(&mut self, _: Self::Message) -> (Command<Messages>, Option<Screens>) {
        run(&self.0).unwrap();
        (Command::none(), None)
    }

    fn view(&self) -> Element<'_, Messages> {
        centering_container(button("Run").on_press(Messages::Main(Message::Run))).into()
    }
}

impl From<Main> for Screens {
    fn from(value: Main) -> Self {
        Screens::Main(value)
    }
}

fn run(launcher_path: &Path) -> Result<(), Box<dyn Error>> {
    println!("Selected path: {:?}", launcher_path);

    add_profile()?;

    Program::new(launcher_path).spawn()?;
    Ok(())
}

fn add_profile() -> Result<(), Box<dyn Error>> {
    let path = dirs::config_dir()
        .map(|path| path.join(".minecraft/launcher_profiles.json"))
        .expect("Path should exist.");
    let mut value = serde_json::from_str::<Value>(&fs::read_to_string(&path)?)?;
    if !value["profiles"].is_object() {
        value["profiles"] = json!({});
    }
    let profiles = &mut value["profiles"];
    profiles["aethon"] = json!({
        "name": "Aethon",
        "type": "custom",
        "icon": "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAIAAAACAAQMAAAD58POIAAAABlBMVEUAAAD4APit1uGJAAAAI0lEQVRIx2P4DwUMMDAqMCowKjAqQKTAaDCMCowKjAqQKQAABpD8LlM5SL4AAAAASUVORK5CYII",
        "lastVersionId": "latest-release"
    });
    fs::write(&path, serde_json::to_string(&value)?)?;
    Ok(())
}
