use iced::advanced::layout::{Limits, Node};
use iced::advanced::renderer::Style;
use iced::advanced::widget::{tree, Operation, Tree};
use iced::advanced::{renderer, Clipboard, Layout, Shell, Widget};
use iced::mouse::{Cursor, Interaction};
use iced::{event, mouse, touch, Element, Event, Length, Rectangle};

pub(crate) struct ClickHoverArea<'a, Message, Renderer> {
    content: Element<'a, Message, Renderer>,
    on_click: Option<Message>,
    on_hover_change: Option<Box<dyn Fn(bool) -> Message + 'a>>,
}

#[derive(Default)]
struct State {
    is_hovered: bool,
}

impl<'a, Message, Renderer> ClickHoverArea<'a, Message, Renderer> {
    pub(crate) fn new(content: impl Into<Element<'a, Message, Renderer>>) -> Self {
        Self {
            content: content.into(),
            on_click: None,
            on_hover_change: None,
        }
    }

    pub(crate) fn on_click(mut self, message: Message) -> Self {
        self.on_click = Some(message);
        self
    }

    pub(crate) fn on_hover_change<F>(mut self, message: F) -> Self
    where
        F: 'a + Fn(bool) -> Message,
    {
        self.on_hover_change = Some(Box::new(message));
        self
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for ClickHoverArea<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
    Message: Clone,
{
    fn width(&self) -> Length {
        self.content.as_widget().width()
    }

    fn height(&self) -> Length {
        self.content.as_widget().height()
    }

    fn layout(&self, renderer: &Renderer, limits: &Limits) -> Node {
        self.content.as_widget().layout(renderer, limits)
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        style: &Style,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
    ) {
        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        )
    }

    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content))
    }

    fn operate(
        &self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<Message>,
    ) {
        self.content
            .as_widget()
            .operate(&mut tree.children[0], layout, renderer, operation);
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        if let event::Status::Captured = self.content.as_widget_mut().on_event(
            &mut tree.children[0],
            event.clone(),
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        ) {
            return event::Status::Captured;
        }

        let is_over = cursor.is_over(layout.bounds());

        if is_over {
            if let Some(message) = self.on_click.as_ref() {
                if let Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
                | Event::Touch(touch::Event::FingerPressed { .. }) = event
                {
                    shell.publish(message.clone());

                    return event::Status::Captured;
                }
            }
        }

        if let Some(message) = self.on_hover_change.as_ref() {
            let state = tree.state.downcast_mut::<State>();
            let old_is_hovered = state.is_hovered;
            state.is_hovered = is_over;
            if old_is_hovered != state.is_hovered {
                shell.publish((message)(state.is_hovered));
                return event::Status::Captured;
            }
        }

        event::Status::Ignored
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> Interaction {
        self.content.as_widget().mouse_interaction(
            &tree.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
    ) -> Option<iced::advanced::overlay::Element<'b, Message, Renderer>> {
        self.content
            .as_widget_mut()
            .overlay(&mut tree.children[0], layout, renderer)
    }
}

impl<'a, Message, Renderer> From<ClickHoverArea<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Renderer: 'a + renderer::Renderer,
    Message: 'a + Clone,
{
    fn from(value: ClickHoverArea<'a, Message, Renderer>) -> Self {
        Element::new(value)
    }
}
