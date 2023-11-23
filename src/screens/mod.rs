use std::fmt::Debug;

use crate::screens::error::Error;
use crate::Errors;
use iced::widget::{container, Container};
use iced::{Command, Element, Length};

use crate::screens::folder_warn::FolderNotEmptyWarn;
use crate::screens::instance_warn::SingleInstanceWarn;
use crate::screens::main::Main;
use crate::screens::setup::Setup;
use crate::screens::startup::Startup;

pub(crate) mod error;
pub(crate) mod folder_warn;
pub(crate) mod instance_warn;
pub(crate) mod main;
pub(crate) mod setup;
pub(crate) mod startup;

pub(crate) fn centering_container<'a, Message>(
    content: impl Into<Element<'a, Message>>,
) -> Container<'a, Message> {
    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .padding(10)
}

pub(crate) trait Screen: Debug + Clone + Into<Screens> {
    type Message: Debug + Send;

    fn update(&mut self, message: Self::Message) -> (Command<Messages>, Option<Screens>);

    fn view(&self) -> Element<'_, Messages>;
}

#[derive(Debug, Clone)]
pub(crate) enum Screens {
    Startup(Startup),
    FolderNotEmptyWarn(FolderNotEmptyWarn),
    SingleInstanceWarn(SingleInstanceWarn),
    Setup(Setup),
    Main(Main),
    Error(Error),
}

#[derive(Debug, Clone)]
pub(crate) enum Messages {
    Save(Result<(), Errors>),
    Startup(startup::Message),
    FolderNotEmptyWarn(folder_warn::Message),
    SingleInstanceWarn(instance_warn::Message),
    Setup(setup::Message),
    Main(main::Message),
    Error(error::Message),
}
