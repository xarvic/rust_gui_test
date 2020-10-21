use druid_shell::MouseEvent;
use druid_shell::kurbo::{Vec2, Rect};

pub enum Event{
    MouseEnter(MouseEvent),
    MouseMove(MouseEvent),
    MouseDown(MouseEvent),
    MouseUp(MouseEvent),
    MouseExit,
}

impl Event {
    pub fn shift(&mut self, amount: Vec2) {
        match self {
            Event::MouseEnter(me) => {me.pos += amount}
            Event::MouseMove(me) => {me.pos += amount}
            Event::MouseDown(me) => {me.pos += amount}
            Event::MouseUp(me) => {me.pos += amount}
            _ => {}
        }
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

enum Change {
    None,
    Content(Rect),
    Bounds,
}

enum EventResponse {
    Consumed(Change),
    
}