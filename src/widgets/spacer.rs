use crate::widgets::Widget;
use druid_shell::kurbo::{Rect, Size};
use crate::event::{EventResponse, Event};
use crate::widget_graph::WidgetContext;
use crate::size::PrefSize;
use druid_shell::piet::Piet;
use crate::state::key::Key;

#[derive(Copy, Clone)]
pub struct Spacer(PrefSize);

impl Spacer {
    pub fn fixed(width: f64, height: f64) -> Self {
        Spacer(PrefSize::fixed((width, height)))
    }
    pub fn flex(width: f64, height: f64) -> Self {
        Spacer(PrefSize::flexible((width, height)))
    }
    pub fn grow() -> Self {
        Spacer(PrefSize::flexible(Size::ZERO))
    }
}
impl<T: Clone> Widget<T> for Spacer{
    fn draw(&mut self, painter: &mut Piet, size: Size, dirty_rect: Rect, context: WidgetContext, data: &T) {}

    fn handle_event(&mut self, event: Event, context: WidgetContext, data: Key<T>) -> EventResponse {
        EventResponse::NONE
    }

    fn get_pref_size(&mut self, context: WidgetContext, data: &T) -> PrefSize {
        self.0
    }

    fn layout(&mut self, size: Size, context: WidgetContext, data: &T) {}

    fn build(&mut self, context: WidgetContext) {}
}