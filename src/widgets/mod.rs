use crate::state::lens::Lens;
use druid_shell::piet::Piet;
use crate::event::Event;
use crate::state::key::Key;
use crate::widgets::layout::SizeConstrains;
use druid_shell::kurbo::Size;

pub mod layout;
pub mod state;
pub mod lens;


pub trait Widget<T: Clone> {
    fn draw(&self, painter: &mut Piet, data: &T);
    fn handle_event(&mut self, event: Event, data: Key<T>);
    fn layout(&mut self, constrains: SizeConstrains, data: &T) -> Size;
    fn traverse_focus(&mut self) -> bool;
}

pub struct Empty;

impl<T: Clone> Widget<T> for Empty {
    fn draw(&self, painter: &mut Piet, data: &T) {
    }

    fn handle_event(&mut self, event: Event, data: Key<T>) {
    }

    fn layout(&mut self, constrains: SizeConstrains, data: &T) -> Size {
        Size::ZERO
    }

    fn traverse_focus(&mut self) -> bool {
        false
    }
}