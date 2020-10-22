use crate::size::PrefSize;
use crate::widgets::Widget;
use druid_shell::kurbo::{Rect, Size};
use crate::event::{EventResponse, Event};
use crate::widget_graph::WidgetContext;
use druid_shell::piet::Piet;
use crate::state::key::Key;

pub struct PrefSizeWrapper<W>{
    widget: W,
    pref_size: PrefSize,
    restrict: bool,
}

impl<W> PrefSizeWrapper<W> {
    pub fn new(widget: W, pref_size: PrefSize, restrict: bool) -> Self {
        PrefSizeWrapper{
            widget,
            pref_size,
            restrict,
        }
    }
}

impl<T: Clone, W: Widget<T>> Widget<T> for PrefSizeWrapper<W> {
    fn draw(&mut self, painter: &mut Piet, size: Size, dirty_rect: Rect, context: WidgetContext, data: &T) {
        self.widget.draw(painter, size, dirty_rect, context, data)
    }

    fn handle_event(&mut self, event: Event, context: WidgetContext, data: Key<T>) -> EventResponse {
        self.widget.handle_event(event, context, data)
    }

    fn get_pref_size(&mut self, context: WidgetContext, data: &T) -> PrefSize {
        let mut pref_size = self.widget.get_pref_size(context, data);
        //pref_size.max_min_size(self.pref_size.min);
        //I not sure if the widget should do this

        if self.restrict {
            pref_size.min_max_size(self.pref_size.max);
            pref_size.max_max_size(pref_size.min);
            pref_size.min_grow(self.pref_size.grow);
        } else {
            pref_size.max_max_size(self.pref_size.max);
            pref_size.max_grow(self.pref_size.grow);
        }

        pref_size
    }

    fn layout(&mut self, size: Size, context: WidgetContext, data: &T) {
        self.widget.layout(size, context, data)
    }

    fn build(&mut self, context: WidgetContext) {
        self.widget.build(context)
    }

    fn traverse_focus(&mut self, context: WidgetContext) -> bool {
        self.widget.traverse_focus(context)
    }
}