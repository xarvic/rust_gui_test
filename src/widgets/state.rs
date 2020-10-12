use crate::state::{StateID, State};
use crate::event::Event;
use druid_shell::piet::Piet;
use crate::widgets::{Widget, Empty};

pub trait StateWidget {
    fn update(&mut self);
    fn draw(&self, painter: &mut Piet);
    fn handle_event(&mut self, event: Event);

    fn states(&self) -> Vec<StateID>;
}

fn state<T: Clone, U: Clone>(state: State<T>, widget: impl Widget<T>) -> impl Widget<U> {
    let x: Empty = unimplemented!();
    x
}

