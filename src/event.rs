use druid_shell::MouseEvent;
use druid_shell::kurbo::{Vec2, Rect, Size};

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