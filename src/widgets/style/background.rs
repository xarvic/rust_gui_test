use crate::widgets::Widget;
use druid_shell::piet::{Color, Piet};
use druid_shell::kurbo::{Rect, Size};
use crate::event::{EventResponse, Event};
use crate::widget_graph::WidgetContext;
use crate::state::key::Key;
use crate::widgets::style::Background;
use crate::size::PrefSize;

pub struct BackgroundWrapper<W> {
    widget: W,
    background: Background,
}

pub fn background<T: Clone, W: Widget<T>>(color: Color, widget: W) -> BackgroundWrapper<W> {
    BackgroundWrapper {
        widget,
        background: Background::from_color(color),
    }
}

impl<T: Clone, W: Widget<T>> Widget<T> for BackgroundWrapper<W> {
    fn draw(&mut self, painter: &mut Piet, size: Size, dirty_rect: Rect, context: WidgetContext, data: &T) {
        self.background.draw(painter, size);
        self.widget.draw(painter, size, dirty_rect, context, data);
    }

    fn handle_event(&mut self, event: Event, context: WidgetContext, data: Key<T>) -> EventResponse {
        self.widget.handle_event(event, context, data)
    }

    fn get_pref_size(&mut self, context: WidgetContext, data: &T) -> PrefSize {
        self.widget.get_pref_size(context, data)
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