use druid_shell::piet::Piet;
use crate::event::{Event, EventResponse};
use crate::state::key::Key;
use druid_shell::kurbo::{Size, Rect};
use crate::widget_graph::WidgetContext;

pub mod layout;
pub mod state;
pub mod lens;

mod text;

pub use text::Label;
pub use layout::PrefSize;

pub trait Widget<T: Clone> {
    fn draw(&mut self, painter: &mut Piet, size: Size, dirty_rect: Rect, context: WidgetContext, data: &T);

    fn handle_event(&mut self, event: Event, context: WidgetContext, data: Key<T>) -> EventResponse;

    fn get_pref_size(&mut self, context: WidgetContext, data: &T) -> PrefSize;

    fn layout(&mut self, size: Size, context: WidgetContext, data: &T);

    fn build(&mut self, context: WidgetContext);

    #[allow(unused_variables)]
    fn traverse_focus(&mut self, context: WidgetContext) -> bool {false}
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
pub struct Empty;

impl<T: Clone> Widget<T> for Empty{
    fn draw(&mut self, painter: &mut Piet, size: Size, dirty_rect: Rect, context: WidgetContext, data: &T) {}

    fn handle_event(&mut self, event: Event, context: WidgetContext, data: Key<T>) -> EventResponse{ EventResponse::Valid }

    fn get_pref_size(&mut self, context: WidgetContext, data: &T) -> PrefSize { PrefSize::fixed(Size::ZERO) }

    fn layout(&mut self, size: Size, context: WidgetContext, data: &T) {}

    fn build(&mut self, context: WidgetContext) {}

    fn traverse_focus(&mut self, context: WidgetContext) -> bool { false }
}


