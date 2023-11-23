//! A container for capturing mouse events.

use iced::advanced::layout;
use iced::advanced::mouse::Cursor::Available;
use iced::advanced::mouse::{Click, Cursor};
use iced::advanced::overlay;
use iced::advanced::renderer;
use iced::advanced::widget::{tree, Operation, Tree};
use iced::advanced::{Clipboard, Layout, Shell, Widget};
use iced::{event, mouse, touch, Element, Event, Length, Point, Rectangle, Vector};
use std::ops::Sub;

/// A container intercepting mouse events.
pub fn mouse_area<'a, Message, Renderer>(
    widget: impl Into<Element<'a, Message, Renderer>>,
) -> MouseArea<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
{
    MouseArea::new(widget)
}

/// Emit messages on mouse events.
#[allow(missing_debug_implementations)]
pub struct MouseArea<'a, Message, Renderer> {
    content: Element<'a, Message, Renderer>,
    on_press: Option<Message>,
    on_release: Option<Message>,
    on_right_press: Option<Message>,
    on_right_release: Option<Message>,
    on_middle_press: Option<Message>,
    on_middle_release: Option<Message>,
    on_enter: Option<Message>,
    on_exit: Option<Message>,
    on_move: Option<Box<dyn Fn(Point) -> Message + 'a>>,
    on_drag: Option<Box<dyn Fn(Vector) -> Message + 'a>>,
    on_click: Option<Box<dyn Fn(Click) -> Message + 'a>>,
}

impl<'a, Message, Renderer> MouseArea<'a, Message, Renderer> {
    /// The message to emit on a left button press.
    #[must_use]
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    /// The message to emit on a left button release.
    #[must_use]
    pub fn on_release(mut self, message: Message) -> Self {
        self.on_release = Some(message);
        self
    }

    /// The message to emit on a right button press.
    #[must_use]
    pub fn on_right_press(mut self, message: Message) -> Self {
        self.on_right_press = Some(message);
        self
    }

    /// The message to emit on a right button release.
    #[must_use]
    pub fn on_right_release(mut self, message: Message) -> Self {
        self.on_right_release = Some(message);
        self
    }

    /// The message to emit on a middle button press.
    #[must_use]
    pub fn on_middle_press(mut self, message: Message) -> Self {
        self.on_middle_press = Some(message);
        self
    }

    /// The message to emit on a middle button release.
    #[must_use]
    pub fn on_middle_release(mut self, message: Message) -> Self {
        self.on_middle_release = Some(message);
        self
    }

    /// The message to emit on mouse enter.
    #[must_use]
    pub fn on_enter(mut self, message: Message) -> Self {
        self.on_enter = Some(message);
        self
    }

    /// The message to emit on mouse exit.
    #[must_use]
    pub fn on_exit(mut self, message: Message) -> Self {
        self.on_exit = Some(message);
        self
    }

    /// The message to emit on mouse move.
    #[must_use]
    pub fn on_move<F>(mut self, callback: F) -> Self
    where
        F: 'a + Fn(Point) -> Message,
    {
        self.on_move = Some(Box::new(callback));
        self
    }

    /// The message to emit on mouse drag.
    #[must_use]
    pub fn on_drag<F>(mut self, callback: F) -> Self
    where
        F: 'a + Fn(Vector) -> Message,
    {
        self.on_drag = Some(Box::new(callback));
        self
    }

    /// The message to emit on mouse click.
    #[must_use]
    pub fn on_click<F>(mut self, callback: F) -> Self
    where
        F: 'a + Fn(Click) -> Message,
    {
        self.on_click = Some(Box::new(callback));
        self
    }
}

/// Local state of the [`MouseArea`].
#[derive(Default)]
struct State {
    last_cursor: Cursor,
    was_over: Option<bool>,
    dragging: bool,
    last_click: Option<Click>,
}

impl<'a, Message, Renderer> MouseArea<'a, Message, Renderer> {
    /// Creates a [`MouseArea`] with the given content.
    pub fn new(content: impl Into<Element<'a, Message, Renderer>>) -> Self {
        MouseArea {
            content: content.into(),
            on_press: None,
            on_release: None,
            on_right_press: None,
            on_right_release: None,
            on_middle_press: None,
            on_middle_release: None,
            on_enter: None,
            on_exit: None,
            on_move: None,
            on_drag: None,
            on_click: None,
        }
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for MouseArea<'a, Message, Renderer>
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

    fn layout(&self, renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
        self.content.as_widget().layout(renderer, limits)
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        renderer_style: &renderer::Style,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
    ) {
        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            renderer_style,
            layout,
            cursor,
            viewport,
        );
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
        tree.diff_children(std::slice::from_ref(&self.content));
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
            return event::Status::Ignored;
        }

        update(
            tree.state.downcast_mut::<State>(),
            self,
            &event,
            layout,
            cursor,
            shell,
        )
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
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
    ) -> Option<overlay::Element<'b, Message, Renderer>> {
        self.content
            .as_widget_mut()
            .overlay(&mut tree.children[0], layout, renderer)
    }
}

impl<'a, Message, Renderer> From<MouseArea<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + renderer::Renderer,
{
    fn from(area: MouseArea<'a, Message, Renderer>) -> Element<'a, Message, Renderer> {
        Element::new(area)
    }
}

/// Processes the given [`Event`] and updates the [`State`] of an [`MouseArea`]
/// accordingly.
fn update<Message: Clone, Renderer>(
    state: &mut State,
    widget: &mut MouseArea<'_, Message, Renderer>,
    event: &Event,
    layout: Layout<'_>,
    cursor: Cursor,
    shell: &mut Shell<'_, Message>,
) -> event::Status {
    let mut status: Option<event::Status> = None;

    let is_cursor_over = cursor.is_over(layout.bounds());

    let mut _cursor = cursor;
    if let Event::Mouse(_) = event {
        if !is_cursor_over {
            state.dragging = false;
            _cursor = Cursor::Unavailable;
        }
    }

    if let Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
    | Event::Touch(touch::Event::FingerPressed { .. }) = event
    {
        if is_cursor_over {
            if let Available(position) = state.last_cursor {
                state.last_click = Some(Click::new(position, state.last_click));

                if let Some(message) = widget.on_click.as_ref() {
                    shell.publish((message)(state.last_click.unwrap()));
                    status = status.or(Some(event::Status::Captured));
                }
            }
            state.dragging = true;

            if let Some(message) = widget.on_press.as_ref() {
                shell.publish(message.clone());
                status = status.or(Some(event::Status::Captured));
            }
        }
    }

    if let Event::Mouse(mouse::Event::CursorMoved { position, .. }) = event {
        if is_cursor_over {
            if let Some(false) = state.was_over {
                if let Some(message) = widget.on_enter.as_ref() {
                    shell.publish(message.clone());
                    status = status.or(Some(event::Status::Captured));
                }
            }

            if let Some(message) = widget.on_move.as_ref() {
                shell.publish((message)(*position));
                status = status.or(Some(event::Status::Captured));
            }

            if state.dragging {
                if let Some(message) = widget.on_drag.as_ref() {
                    let delta =
                        position.sub(state.last_cursor.position().unwrap_or(Point::new(0.0, 0.0)));
                    shell.publish((message)(Vector::new(delta.x, delta.y)));
                    status = status.or(Some(event::Status::Captured));
                }
            }

            state.last_cursor = Available(*position);
            state.was_over = Some(true);
        } else {
            if let Some(true) = state.was_over {
                if let Some(message) = widget.on_exit.as_ref() {
                    shell.publish(message.clone());
                    status = status.or(Some(event::Status::Captured));
                }
            }

            state.last_cursor = Cursor::Unavailable;
            state.was_over = Some(false);
        }
    }

    if let Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
    | Event::Touch(touch::Event::FingerLifted { .. }) = event
    {
        if is_cursor_over {
            state.dragging = false;
            if let Some(message) = widget.on_release.as_ref() {
                shell.publish(message.clone());
                status = status.or(Some(event::Status::Captured));
            }
        }
    }

    if let Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Right)) = event {
        if is_cursor_over {
            if let Some(message) = widget.on_right_press.as_ref() {
                shell.publish(message.clone());
                status = status.or(Some(event::Status::Captured));
            }
        }
    }

    if let Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Right)) = event {
        if is_cursor_over {
            if let Some(message) = widget.on_right_release.as_ref() {
                shell.publish(message.clone());
                status = status.or(Some(event::Status::Captured));
            }
        }
    }

    if let Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Middle)) = event {
        if is_cursor_over {
            if let Some(message) = widget.on_middle_press.as_ref() {
                shell.publish(message.clone());
                status = status.or(Some(event::Status::Captured));
            }
        }
    }

    if let Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Middle)) = event {
        if is_cursor_over {
            if let Some(message) = widget.on_middle_release.as_ref() {
                shell.publish(message.clone());
                status = status.or(Some(event::Status::Captured));
            }
        }
    }

    status.unwrap_or(event::Status::Ignored)
}
