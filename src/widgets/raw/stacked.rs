use crate::widgets::Widget;
use druid_shell::kurbo::{Rect, Size};
use crate::event::{EventResponse, Event};
use crate::widget_graph::WidgetContext;
use druid_shell::piet::{Piet, RenderContext};
use crate::state::key::Key;
use crate::size::PrefSize;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum AB {
    A,
    B,
}

pub struct Stacked<A, B> {
    widget_top: A,
    widget_bottom: B,
    focus: Option<AB>,
    mouse_focus: Option<AB>,
}

pub fn statcked<A, B>(widget_top: A, widget_bottom: B) -> Stacked<A, B> {
    Stacked{
        widget_top,
        widget_bottom,
        focus: None,
        mouse_focus: None
    }
}

impl<T: Clone, A: Widget<T>, B: Widget<T>> Widget<T> for Stacked<A, B> {
    fn draw(&mut self, painter: &mut Piet, size: Size, dirty_rect: Rect, mut context: WidgetContext, data: &T) {
        let _ = painter.with_save(|piet|{
            self.widget_bottom.draw(piet, size, dirty_rect, context.id(), data);
            Ok(())
        });
        self.widget_top.draw(painter, size, dirty_rect, context.id(), data);
    }

    fn handle_event(&mut self, event: Event, mut context: WidgetContext, mut data: Key<T>) -> EventResponse {
        let mut response = self.widget_top.handle_event(event.clone(), context.id(), data.id());
        if response.is_present() {
            response = response.merge(self.widget_bottom.handle_event(event, context, data));
        }
        response
    }

    fn get_pref_size(&mut self, mut context: WidgetContext, data: &T) -> PrefSize {
        let pref = self.widget_top.get_pref_size(context.id(), data);
        pref.stacked(self.widget_bottom.get_pref_size(context, data))
    }

    fn layout(&mut self, size: Size, mut context: WidgetContext, data: &T) {
        self.widget_top.layout(size, context.id(), data);
        self.widget_bottom.layout(size, context, data);
    }

    fn build(&mut self, mut context: WidgetContext) {
        self.widget_top.build(context.id());
        self.widget_bottom.build(context.id());
    }

    fn traverse_focus(&mut self, mut context: WidgetContext) -> bool {
        if self.focus == None || self.focus == Some(AB::A) {
            if self.widget_top.traverse_focus(context.id()) {
                self.focus = Some(AB::A);
                return true;
            } else {
                self.focus = Some(AB::B);
            }
        }
        if self.focus == Some(AB::B) {
            if self.widget_bottom.traverse_focus(context.id()) {
                return true;
            }
        }
        self.focus = None;
        return false;
    }
}