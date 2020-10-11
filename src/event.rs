use druid_shell::MouseEvent;
use druid_shell::kurbo::Vec2;

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
}