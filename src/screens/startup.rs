use std::env;
use std::path::Path;

use iced::widget::text;
use iced::{Command, Element};

use crate::screens::folder_warn::FolderNotEmptyWarn;
use crate::screens::instance_warn::SingleInstanceWarn;
use crate::screens::setup::load_setup;
use crate::screens::{centering_container, Messages, Screen, Screens};
use crate::INSTANCE;

#[derive(Debug, Clone)]
pub(crate) struct Startup;

#[derive(Debug, Clone)]
pub(crate) enum Message {
    Loaded(Screens),
}

impl Screen for Startup {
    type Message = Message;

    fn update(&mut self, message: Self::Message) -> (Command<Messages>, Option<Screens>) {
        let Message::Loaded(screen) = message;
        (Command::none(), Some(screen))
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

pub(crate) async fn load() -> Screens {
    if !INSTANCE.is_single() {
        return SingleInstanceWarn.into();
    }
    if env::current_exe()
        .ok()
        .as_deref()
        .and_then(Path::parent)
        .map(Path::read_dir)
        .and_then(Result::ok)
        .map(|dir| dir.filter_map(Result::ok))
        .map(|dir| dir.count() > 1)
        .unwrap_or(true)
    {
        return FolderNotEmptyWarn.into();
    }
    load_setup().await.into()
}
