use crate::state::{StateID, CloneState, State};
use druid_shell::piet::Piet;
use crate::widgets::widget::Widget;
use druid_shell::kurbo::{Size, Rect};
use crate::state::key::Key;
use pool_tree::child_unique::ChildUniq;
use std::mem::replace;
use crate::app::event::{Change, Event, EventResponse};
use crate::app::widget_graph::{WidgetContext, StateRoot, Env};
use crate::app::size::PrefSize;

///
pub trait StateWidget {
    fn update(&mut self) -> Change;
    fn draw(&mut self, painter: &mut Piet, size: Size, dirty_rect: Rect, context: WidgetContext);
    fn handle_event(&mut self, event: Event, context: WidgetContext) -> EventResponse;
    fn get_pref_size(&mut self, context: WidgetContext) -> PrefSize;
    fn layout(&mut self, size: Size, context: WidgetContext);
    fn build(&mut self, context: WidgetContext);
    fn traverse_focus(&mut self, context: WidgetContext) -> bool;

    fn states(&self) -> StateID;
}

pub enum StateWrapper{
    Tree(u32),
    Local(Box<dyn StateWidget>),
}

impl StateWrapper{
    fn as_child<R>(&mut self, mut context: WidgetContext, operation: impl FnOnce(ChildUniq<StateRoot>, Env) -> R) -> R {
        match replace(self, StateWrapper::Tree(0)) {
            StateWrapper::Tree(index) => {
                *self = StateWrapper::Tree(index);
                let (a, b) = context.get_child(index);
                operation(a, b)
            }
            StateWrapper::Local(value) => {
                let (a, b, c) = context.insert(StateRoot::new(value));
                *self = StateWrapper::Tree(c);
                operation(a, b)
            }
        }
    }
}


impl<T: Clone> Widget<T> for StateWrapper {
    fn draw(&mut self, painter: &mut Piet, size: Size, dirty_rect: Rect, context: WidgetContext, data: &T) {
        self.as_child(context, move|child, env|child.draw(painter, size, dirty_rect, env))
    }

    fn handle_event(&mut self, event: Event, context: WidgetContext, data: Key<T>) -> EventResponse {
        self.as_child(context, move|child, env|child.handle_event(event, env))
    }

    fn get_pref_size(&mut self, context: WidgetContext, data: &T) -> PrefSize {
        self.as_child(context, move|child, env|child.get_pref_size(env))
    }

    fn layout(&mut self, size: Size, context: WidgetContext, data: &T) {
        self.as_child(context, move|child, env|child.layout(size, env))
    }

    fn build(&mut self, context: WidgetContext) {
        self.as_child(context, move|child, env|child.build(env))
    }

    fn update(&mut self, new: &T, old: Option<&T>) -> Change {
        //The wrapped state will get its own update if its state changed!
        Change::None
    }

    fn traverse_focus(&mut self, context: WidgetContext) -> bool {
        self.as_child(context, move|child, env|child.traverse_focus(env))
    }
}

pub fn state<T: Clone + Send + Sync + 'static>(state: &CloneState<T>, widget: impl Widget<T> + 'static) -> StateWrapper {
    StateWrapper::Local(
        Box::new(
            StateWidgetImpl {
                state: state.clone(),
                widget,
            }
        )
    )
}

pub(crate) struct StateWidgetImpl<T: Clone, W: Widget<T>> {
    state: CloneState<T>,
    widget: W,
}

impl<T: Clone + Send + Sync + 'static, W: Widget<T>> StateWidgetImpl<T, W> {
    pub(crate) fn new(state: CloneState<T>, widget: W) -> Self {
        StateWidgetImpl {
            state,
            widget,
        }
    }
}

impl<T: Clone + Send + Sync + 'static, W: Widget<T>> StateWidget for StateWidgetImpl<T, W> {
    fn update(&mut self) -> Change {
        let widget = &mut self.widget;
        self.state.with_fetched_value(|new, old|widget.update(new, old))
    }

    fn draw(&mut self, painter: &mut Piet, size: Size, dirty_rect: Rect, context: WidgetContext) {
        let widget = &mut self.widget;
        self.state.with_value(|value|widget.draw(painter, size, dirty_rect, context, value))
    }

    fn handle_event(&mut self, event: Event, context: WidgetContext) -> EventResponse{
        let widget = &mut self.widget;
        self.state.with_key(|value|widget.handle_event(event, context, value))
    }

    fn get_pref_size(&mut self, context: WidgetContext) -> PrefSize {
        let widget = &mut self.widget;
        self.state.with_value(|value|widget.get_pref_size(context, value))
    }

    fn layout(&mut self, size: Size, context: WidgetContext) {
        let widget = &mut self.widget;
        self.state.with_value(|value|widget.layout(size, context, value))
    }

    fn build(&mut self, context: WidgetContext) {
        self.widget.build(context)
    }

    fn traverse_focus(&mut self, context: WidgetContext) -> bool {
        self.widget.traverse_focus(context)
    }

    fn states(&self) -> StateID {
        self.state.get_id()
    }
}

/*pub fn fixed<T: Clone + 'static, U: Clone>(data: T, widget: impl Widget<T> + 'static) -> StateWrapper {
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
}*/