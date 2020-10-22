use druid_shell::piet::Piet;
use druid_shell::kurbo::{Size, Rect};
use crate::widget_graph::WidgetContext;
use crate::event::{Event, EventResponse};
use crate::size::PrefSize;
use crate::state::key::Key;
use crate::widgets::raw::{Wrapper, WrapperWidget};

pub trait Widget<T: Clone> {
    fn draw(&mut self, painter: &mut Piet, size: Size, dirty_rect: Rect, context: WidgetContext, data: &T);

    fn handle_event(&mut self, event: Event, context: WidgetContext, data: Key<T>) -> EventResponse;

    fn get_pref_size(&mut self, context: WidgetContext, data: &T) -> PrefSize;

    fn layout(&mut self, size: Size, context: WidgetContext, data: &T);

    fn build(&mut self, context: WidgetContext);

    #[allow(unused_variables)]
    fn traverse_focus(&mut self, context: WidgetContext) -> bool {false}


}

impl<T: Clone, W: Widget<T>> WidgetCompose<T> for W {

}

pub trait WidgetCompose<T: Clone>: Widget<T> where Self: Sized {
    fn wrap<W: Wrapper>(self, wrapper: W) -> WrapperWidget<W, Self> {
        WrapperWidget::new(wrapper, self)
    }
}