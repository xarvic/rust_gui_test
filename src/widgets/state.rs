use crate::state::{StateID, CloneState};
use crate::event::{Event, EventResponse};
use druid_shell::piet::Piet;
use crate::widgets::widget::Widget;
use druid_shell::kurbo::{Size, Rect};
use crate::state::key::Key;
use crate::widget_graph::WidgetContext;
use std::any::Any;
use crate::size::PrefSize;

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
    fn draw(&mut self, painter: &mut Piet, size: Size, dirty_rect: Rect, context: WidgetContext, data: &T) {
        unimplemented!()
    }

    fn handle_event(&mut self, event: Event, context: WidgetContext, data: Key<T>) -> EventResponse {
        unimplemented!()
    }

    fn get_pref_size(&mut self, context: WidgetContext, data: &T) -> PrefSize {
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

fn state<T: Clone + 'static, U: Clone>(state: CloneState<T>, widget: impl Widget<T> + 'static) -> StateWrapper {
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
    state: CloneState<T>,
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