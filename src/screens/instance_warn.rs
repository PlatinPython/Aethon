use std::process;

use iced::widget::{button, column, container, horizontal_space, row, text};
use iced::{Command, Element, Length};

use crate::screens::{centering_container, Messages, Screen, Screens};

#[derive(Debug, Clone)]
pub(crate) struct SingleInstanceWarn;

#[derive(Debug, Clone)]
pub(crate) enum Message {
    Close,
}

impl Screen for SingleInstanceWarn {
    type Message = Message;

    fn update(&mut self, _: Self::Message) -> (Command<Messages>, Option<Screens>) {
        process::exit(0);
    }

    fn view(&self) -> Element<'_, Messages> {
        centering_container({
            column![
                container(text("Another instance is already running."))
                    .width(Length::Fill)
                    .center_x(),
                row![
                    horizontal_space(Length::FillPortion(2)),
                    button("Ok")
                        .width(Length::Fill)
                        .on_press(Messages::SingleInstanceWarn(Message::Close)),
                    horizontal_space(Length::FillPortion(2)),
                ]
                .spacing(10)
            ]
            .spacing(10)
        })
        .into()
    }
}

impl From<SingleInstanceWarn> for Screens {
    fn from(value: SingleInstanceWarn) -> Self {
        Screens::SingleInstanceWarn(value)
    }
}
