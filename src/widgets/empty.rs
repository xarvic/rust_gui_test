use crate::widgets::Widget;
use druid_shell::piet::Piet;
use druid_shell::kurbo::{Size, Rect};
use crate::widget_graph::WidgetContext;
use crate::event::{EventResponse, Event};
use crate::state::key::Key;
use crate::size::PrefSize;

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
pub struct Empty;

impl<T: Clone> Widget<T> for Empty{
    fn draw(&mut self, painter: &mut Piet, size: Size, dirty_rect: Rect, context: WidgetContext, data: &T) {}

    fn handle_event(&mut self, event: Event, context: WidgetContext, data: Key<T>) -> EventResponse{ EventResponse::NONE }

    fn get_pref_size(&mut self, context: WidgetContext, data: &T) -> PrefSize {
        let mut size = PrefSize::fixed(Size::ZERO);
        size.grow_needed_x();
        size.grow_needed_y();
        size
    }

    fn layout(&mut self, size: Size, context: WidgetContext, data: &T) {}

    fn build(&mut self, context: WidgetContext) {}

    fn traverse_focus(&mut self, context: WidgetContext) -> bool { false }
}

