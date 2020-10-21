use druid_shell::MouseEvent;
use druid_shell::kurbo::{Vec2, Rect, Size};
use crate::state::key::Key;
use crate::widgets::{Widget, PrefSize};
use crate::widget_graph::WidgetContext;
use druid_shell::piet::Piet;

#[derive(Clone, PartialEq)]
pub enum Event{
    MouseEnter(MouseEvent),
    MouseMove(MouseEvent),
    MouseDown(MouseEvent),
    MouseUp(MouseEvent),
    MouseExit,
}

impl Event {
    pub fn shift(&mut self, amount: Vec2, new_size: Size) -> bool {
        let me = match self {
            Event::MouseEnter(me) => {me}
            Event::MouseMove(me) => {me}
            Event::MouseDown(me) => {me}
            Event::MouseUp(me) => {me}
            _ => {return true}
        };
        me.pos -= amount;
        me.pos.x >= 0.0 && me.pos.y >= 0.0 && new_size.width >= me.pos.x && new_size.height >= me.pos.y
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

#[derive(Copy, Clone)]
pub enum Change {
    None,
    Content(Rect),
    Bounds,
}


impl Change {
    pub fn merge(&self, other: Self) -> Self {
        match (self, other) {
            (Change::Bounds, _) => Change::Bounds,
            (_, Change::Bounds) => Change::Bounds,
            (Change::Content(rect), Change::Content(rect2)) => Change::Content(rect.union(rect2)),
            (Change::Content(rect), _) => Change::Content(*rect),
            (_, Change::Content(rect)) => Change::Content(rect),
            (_, _) => Change::None,
        }
    }
    pub fn changed(&self) -> bool {
        match self {
            Change::None => {false}
            Change::Content(_) => {true}
            Change::Bounds => {true}
        }
    }
    pub fn bounds_changed(&self) -> bool {
        match self {
            Change::None => {false}
            Change::Content(_) => {false}
            Change::Bounds => {true}
        }
    }
    pub fn shift(mut self, amount: Vec2) -> Self {
        match &mut self {
            Change::None => {Change::None}
            Change::Content(rect) => {Change::Content(Rect::new(rect.x0 + amount.x, rect.y0 + amount.y, rect.x1 + amount.x, rect.y1 + amount.y))}
            Change::Bounds => {Change::Bounds}
        }
    }
}

#[derive(Copy, Clone)]
#[must_use]
pub struct EventResponse {
    change: Change,
    consumed: bool,
}


impl EventResponse {
    pub const NONE: EventResponse = Self::unchanged(false);
    pub const DELETED: EventResponse = Self::unchanged(true);

    pub const fn changed(size: Size, consumed: bool) -> Self {
        Self {
            change: Change::Content(Rect::new(0.0, 0.0, size.width, size.height)),
            consumed,
        }
    }
    pub const fn unchanged(consumed: bool) -> Self {
        Self {
            change: Change::None,
            consumed,
        }
    }
    pub const fn bounds_changed(consumed: bool) -> Self {
        Self {
            change: Change::Bounds,
            consumed,
        }
    }

    pub fn dirty_rect(&self) -> Option<Rect> {
        match self.change {
            Change::None => {None}
            Change::Content(rect) => {Some(rect)}
            Change::Bounds => {None}
        }
    }


    pub fn merge(&self, other: Self) -> Self{
        EventResponse {
            consumed: self.consumed || other.consumed,
            change: self.change.merge(other.change),
        }
    }
    pub fn shift(&self, amount: Vec2) -> Self {
        Self {
            change: self.change.shift(amount),
            consumed: self.consumed,
        }
    }
    pub fn is_present(&self) -> bool {
        !self.consumed
    }
    pub fn is_consumed(&self) -> bool {
        !self.consumed
    }
    pub fn change(&self) -> Change {
        self.change
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
                return EventResponse::DELETED;
            },
            Event::MouseExit => {
                pressed = false;
                return EventResponse::DELETED;
            },
            Event::MouseUp(event) => {
                if pressed {
                    pressed = false;
                    listener(event, data);
                    return EventResponse::DELETED;
                }
            },
            _ => {},
        }
        EventResponse::NONE
    })
}

pub fn event_listener<T: Clone, W: Widget<T>>(widget: W, mut listener: impl FnMut(Event, Key<T>) -> EventResponse) -> impl Widget<T> {
    EventModifier(widget, move|event: Event, widget: &mut W, context: WidgetContext, mut data: Key<T>|{
        let mut respone = widget.handle_event(event.clone(), context, data.id());
        if respone.is_present() {
            respone = respone.merge(listener(event, data))
        }
        respone
    })
}
pub fn event_filter<T: Clone, W: Widget<T>>(widget: W, mut listener: impl FnMut(Event, Key<T>) -> EventResponse) -> impl Widget<T> {
    EventModifier(widget, move|event: Event, widget: &mut W, context: WidgetContext, mut data: Key<T>|{
        let mut response = listener(event.clone(), data.id());
        if response.is_present() {
            response = response.merge(widget.handle_event(event, context, data))
        }
        response
    })
}

