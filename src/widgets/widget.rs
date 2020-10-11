use druid_shell::piet::Piet;
use crate::event::Event;
use crate::state::StateID;
use druid_shell::kurbo::Size;
use crate::widgets::layout::SizeConstrains;
use crate::state::key::Key;
use crate::state::lens::Lens;

pub trait Widget<F, T: Clone> where F: Lens<T> {
    fn draw(&self, painter: &mut Piet, data: &T);
    fn handle_event(&mut self, event: Event, data: Key<F, T>);
    fn layout(&mut self, constrains: SizeConstrains, data: &T) -> Size;
    fn traverse_focus(&mut self) -> bool;
}

pub trait StateWidget {
    fn update(&mut self);
    fn draw(&self, painter: &mut Piet);
    fn handle_event(&mut self, event: Event);

    fn states(&self) -> Vec<StateID>;
}