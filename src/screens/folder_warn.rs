use std::process;

use crate::Errors;
use iced::widget::{button, column, container, horizontal_space, row, text};
use iced::{Command, Element, Length};

use crate::screens::error::Error;
use crate::screens::setup::{load_setup, Setup};
use crate::screens::{centering_container, Messages, Screen, Screens};

#[derive(Debug, Clone)]
pub(crate) struct FolderNotEmptyWarn;

#[derive(Debug, Clone)]
pub(crate) enum Message {
    Loaded(Result<Setup, Errors>),
    Continue,
    Close,
}
impl Screen for FolderNotEmptyWarn {
    type Message = Message;

    fn update(&mut self, message: Self::Message) -> (Command<Messages>, Option<Screens>) {
        match message {
            Message::Loaded(result) => (
                Command::none(),
                match result {
                    Ok(screen) => Some(screen.into()),
                    Err(error) => Some(Screens::Error(Error::new(
                        error,
                        Box::new(Screens::FolderNotEmptyWarn(self.clone())),
                    ))),
                },
            ),
            Message::Continue => (
                Command::perform(load_setup(), |result| {
                    Messages::FolderNotEmptyWarn(Message::Loaded(result))
                }),
                None,
            ),
            Message::Close => process::exit(0),
        }
    }

    fn view(&self) -> Element<'_, Messages> {
        centering_container({
            column![container(text("Current folder is not empty, please move the executable to an empty folder (Recommended) or continue anyway (Not recommended).")).width(Length::Fill).center_x(),
                        row![
                            horizontal_space(Length::FillPortion(2)),
                            button("Continue").width(Length::Fill).on_press(Messages::FolderNotEmptyWarn(Message::Continue)),
                            horizontal_space(Length::Fill),
                            button("Close").width(Length::Fill).on_press(Messages::FolderNotEmptyWarn(Message::Close)),
                            horizontal_space(Length::FillPortion(2)),
                        ].spacing(10)
                    ].spacing(10)
        }).into()
    }
}

impl From<FolderNotEmptyWarn> for Screens {
    fn from(value: FolderNotEmptyWarn) -> Self {
        Screens::FolderNotEmptyWarn(value)
    }
}
