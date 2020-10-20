use crate::state::{StateID, State};
use crate::event::Event;
use druid_shell::piet::Piet;
use crate::widgets::Widget;
use druid_shell::kurbo::{Size, Rect};
use crate::state::key::Key;
use crate::widgets::PrefSize;
use crate::widget_graph::WidgetContext;
use std::any::Any;

pub trait StateRootWidget {
    fn update(&mut self) -> bool;
    fn draw(&self, painter: &mut Piet, context: WidgetContext);
    fn handle_event(&mut self, event: Event, context: WidgetContext);
    fn get_pref_size(&self, context: WidgetContext) -> PrefSize;
    fn layout(&mut self, size: Size, context: WidgetContext);
    fn build(&mut self, context: WidgetContext);
    fn traverse_focus(&mut self, context: WidgetContext) -> bool;

    fn states(&self) -> &[StateID];
}

pub enum StateWrapper{
    Tree(u32),
    Local(Box<dyn Any>),//TODO: change to dyn StateRootWidget
}


impl<T: Clone> Widget<T> for StateWrapper {
    fn draw(&self, painter: &mut Piet, dirty_rect: Rect, context: WidgetContext, data: &T) {
        unimplemented!()
    }

    fn handle_event(&mut self, event: Event, context: WidgetContext, data: Key<T>) {
        unimplemented!()
    }

    fn get_pref_size(&self, context: WidgetContext, data: &T) -> PrefSize {
        unimplemented!()
    }

    fn layout(&mut self, size: Size, context: WidgetContext, data: &T) {
        unimplemented!()
    }

    fn build(&mut self, context: WidgetContext) {
        unimplemented!()
    }

    fn traverse_focus(&mut self, context: WidgetContext) -> bool {
        unimplemented!()
    }
}

fn state<T: Clone + 'static, U: Clone>(state: State<T>, widget: impl Widget<T> + 'static) -> StateWrapper {
    StateWrapper::Local(
        Box::new(
            StateWidget {
                state,
                widget,
            }
        )
    )
}

struct StateWidget<T: Clone, W: Widget<T>> {
    state: State<T>,
    widget: W,
}

impl<T: Clone, W: Widget<T>> StateRootWidget for StateWidget<T, W> {
    fn update(&mut self) -> bool {
        unimplemented!()
    }

    fn draw(&self, painter: &mut Piet, context: WidgetContext) {
        unimplemented!()
    }

    fn handle_event(&mut self, event: Event, context: WidgetContext) {
        unimplemented!()
    }

    fn get_pref_size(&self, context: WidgetContext) -> PrefSize {
        unimplemented!()
    }

    fn layout(&mut self, size: Size, context: WidgetContext) {
        unimplemented!()
    }

    fn build(&mut self, context: WidgetContext) {
        unimplemented!()
    }

    fn traverse_focus(&mut self, context: WidgetContext) -> bool {
        unimplemented!()
    }

    fn states(&self) -> &[StateID] {
        unimplemented!()
    }
}

pub fn fixed<T: Clone + 'static, U: Clone>(data: T, widget: impl Widget<T> + 'static) -> StateWrapper {
    StateWrapper::Local(
        Box::new(
            StaticWidget{
                data,
                widget,
            }
        )
    )
}

struct StaticWidget<T: Clone, W: Widget<T>> {
    data: T,
    widget: W,
}