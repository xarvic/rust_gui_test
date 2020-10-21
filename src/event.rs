use druid_shell::MouseEvent;
use druid_shell::kurbo::{Vec2, Rect, Size};
use crate::state::key::Key;
use crate::widgets::{Widget, PrefSize};
use crate::widget_graph::WidgetContext;
use druid_shell::piet::Piet;
use either::Either;

#[derive(Clone, PartialEq)]
pub enum Event{
    MouseEnter(MouseEvent),
    MouseMove(MouseEvent),
    MouseDown(MouseEvent),
    MouseUp(MouseEvent),
    MouseExit,
}

impl Event {
    pub fn shift(&mut self, amount: Vec2) -> bool {
        let me = match self {
            Event::MouseEnter(me) => {me}
            Event::MouseMove(me) => {me}
            Event::MouseDown(me) => {me}
            Event::MouseUp(me) => {me}
            _ => {return true}
        };
        me.pos -= amount;
        me.pos.x >= 0.0 && me.pos.y >= 0.0
    }
    pub fn mouse_event(&self) -> Option<&MouseEvent> {
        match self {
            Event::MouseEnter(me) => {Some(me)}
            Event::MouseMove(me) => {Some(me)}
            Event::MouseDown(me) => {Some(me)}
            Event::MouseUp(me) => {Some(me)}
            _ => None
        }
    }
}

pub enum Change {
    None,
    Content(Rect),
    Bounds,
}

pub enum EventResponse {
    Consumed(Change),
    Valid,
}

impl EventResponse {
    pub fn merge(&mut self, other: Self) {
        //TODO: implement
    }
}

impl<T: Clone, W: Widget<T>, F: FnMut(Event, &mut W, WidgetContext, Key<T>) -> EventResponse> Widget<T> for EventModifier<W, F> {
    fn draw(&mut self, painter: &mut Piet, size: Size, dirty_rect: Rect, context: WidgetContext, data: &T) {
        self.0.draw(painter, size, dirty_rect, context, data)
    }

    fn handle_event(&mut self, event: Event, context: WidgetContext, data: Key<T>) -> EventResponse{
        self.1(event, &mut self.0, context, data)
    }

    fn get_pref_size(&mut self, context: WidgetContext, data: &T) -> PrefSize {
        self.0.get_pref_size(context, data)
    }

    fn layout(&mut self, size: Size, context: WidgetContext, data: &T) {
        self.0.layout(size, context, data)
    }

    fn build(&mut self, context: WidgetContext) {
        self.0.build(context)
    }

    fn traverse_focus(&mut self, context: WidgetContext) -> bool {
        self.0.traverse_focus(context)
    }
}

pub struct EventModifier<W, F> (W, F);

pub fn click_listener<T: Clone, W: Widget<T>>(widget: W, mut listener: impl FnMut(MouseEvent, Key<T>)) -> impl Widget<T> {
    let mut pressed = false;
    event_listener(widget, move|event, data|{
        match event {
            Event::MouseDown(_) => {
                pressed = true;
                return EventResponse::Consumed(Change::None);
            },
            Event::MouseExit => pressed = {
                pressed = false;
                return EventResponse::Consumed(Change::None);
            },
            Event::MouseUp(event) => {
                if pressed {
                    pressed = false;
                    listener(event, data);
                    return EventResponse::Consumed(Change::None);
                }
            },
            _ => {},
        }
        EventResponse::Valid
    })
}

pub fn event_listener<T: Clone, W: Widget<T>>(widget: W, mut listener: impl FnMut(Event, Key<T>) -> EventResponse) -> impl Widget<T> {
    EventModifier(widget, move|event: Event, widget: &mut W, context: WidgetContext, mut data: Key<T>|{
        match widget.handle_event(event.clone(), context, data.id()) {
            EventResponse::Consumed(message) => {
                EventResponse::Consumed(message)
            }
            EventResponse::Valid => {
                listener(event, data)
            }
        }
    })
}
pub fn event_filter<T: Clone, W: Widget<T>>(widget: W, mut listener: impl FnMut(Event, Key<T>) -> Either<Event, EventResponse>) -> impl Widget<T> {
    EventModifier(widget, move|event: Event, widget: &mut W, context: WidgetContext, mut data: Key<T>|{
        listener(event, data.id()).right_or_else(|event|{
            widget.handle_event(event, context, data)
        })
    })
}

