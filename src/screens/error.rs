use iced::widget::{button, column, container, horizontal_space, row, text};
use iced::{Command, Element, Length};

use crate::screens::{centering_container, Messages, Screen, Screens};
use crate::Errors;

#[derive(Debug, Clone)]
pub(crate) struct Error {
    error: Errors,
    previous_screen: Box<Screens>,
}

#[derive(Debug, Clone)]
pub(crate) enum Message {
    Continue,
}

impl Error {
    pub(crate) fn new(error: Errors, previous_screen: Box<Screens>) -> Error {
        Error {
            error,
            previous_screen,
        }
    }
}

impl Screen for Error {
    type Message = Message;

    fn update(&mut self, message: Self::Message) -> (Command<Messages>, Option<Screens>) {
        match message {
            Message::Continue => (Command::none(), Some((*self.previous_screen).clone())),
        }
    }

    fn view(&self) -> Element<'_, Messages> {
        match &self.error {
            Errors::Io(error_kind) => centering_container(
                column![
                    container(text(error_kind)).width(Length::Fill).center_x(),
                    row![
                        horizontal_space(Length::FillPortion(2)),
                        button(container("Continue").width(Length::Fill).center_x())
                            .width(Length::Fill)
                            .on_press(Messages::Error(Message::Continue)),
                        horizontal_space(Length::FillPortion(2)),
                    ]
                ]
                .spacing(10),
            )
            .into(),
            Errors::Json(error) => centering_container(
                column![
                    container(text(error)).width(Length::Fill).center_x(),
                    row![
                        horizontal_space(Length::FillPortion(2)),
                        button(container("Continue").width(Length::Fill).center_x())
                            .width(Length::Fill)
                            .on_press(Messages::Error(Message::Continue)),
                        horizontal_space(Length::FillPortion(2)),
                    ]
                ]
                .spacing(10),
            )
            .into(),
            Errors::NoParent => centering_container(
                column![
                    container(text("No parent")).width(Length::Fill).center_x(),
                    row![
                        horizontal_space(Length::FillPortion(2)),
                        button(container("Continue").width(Length::Fill).center_x())
                            .width(Length::Fill)
                            .on_press(Messages::Error(Message::Continue)),
                        horizontal_space(Length::FillPortion(2)),
                    ]
                ]
                .spacing(10),
            )
            .into(),
        }
    }
}

impl From<Error> for Screens {
    fn from(value: Error) -> Self {
        Screens::Error(value)
    }
}
