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
    pub fn shift(&mut self, amount: Vec2) -> bool {
        let mut me = match self {
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

enum Change {
    None,
    Content(Rect),
    Bounds,
}

enum EventResponse {
    Consumed(Change),
    
}