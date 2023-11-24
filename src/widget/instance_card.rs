use iced::alignment::Horizontal;
use iced::widget::container::Appearance;
use iced::widget::{button, column, component, container, vertical_space, Component};
use iced::{Alignment, Background, Color, Element, Length, Renderer, Theme};

use crate::instance::Instance;
use crate::widget::click_hover_area::ClickHoverArea;

pub(crate) struct InstanceCard<'a, Message>
where
    Message: Clone,
{
    on_run: Message,
    is_hovered: bool,
    width: Length,
    height: Length,
    instance: &'a Instance,
}

#[derive(Debug, Clone)]
pub(crate) enum Event {
    Click,
    Update(bool),
    Run,
}

impl<'a, Message> InstanceCard<'a, Message>
where
    Message: Clone,
{
    pub(crate) fn new(on_run: Message, is_hovered: bool, instance: &'a Instance) -> Self {
        Self {
            on_run,
            is_hovered,
            width: Length::Shrink,
            height: Length::Shrink,
            instance,
        }
    }

    pub(crate) fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    pub(crate) fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }
}

impl<'a, Message> Component<Message, Renderer> for InstanceCard<'a, Message>
where
    Message: Clone,
{
    type State = ();
    type Event = Event;

    fn update(&mut self, _: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            Event::Click => None,
            Event::Update(is_hovered) => {
                self.is_hovered = is_hovered;

                None
            }
            Event::Run => Some(self.on_run.clone()),
        }
    }

    fn view(&self, _: &Self::State) -> Element<'_, Self::Event, Renderer> {
        let bottom: Element<Self::Event> = if self.is_hovered {
            button("Run").on_press(Event::Run).into()
        } else {
            self.instance.name().into()
        };

        ClickHoverArea::new(
            container(
                column!["Sample text", vertical_space(Length::Fill), bottom]
                    .padding(10)
                    .spacing(10)
                    .align_items(Alignment::Center),
            )
            .width(self.width)
            .height(self.height)
            .align_x(Horizontal::Center)
            .style(|_: &Theme| Appearance {
                background: Some(Background::Color(Color::from_rgb8(0x15, 0x17, 0x19))),
                ..Default::default()
            }),
        )
        .on_click(Event::Click)
        .on_hover_change(Event::Update)
        .into()
    }
}

impl<'a, Message> From<InstanceCard<'a, Message>> for Element<'a, Message, Renderer>
where
    Message: 'a + Clone,
{
    fn from(value: InstanceCard<'a, Message>) -> Self {
        component(value)
    }
}
