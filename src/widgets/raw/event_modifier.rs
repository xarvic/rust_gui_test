use crate::widgets::Widget;
use crate::state::key::Key;
use druid_shell::kurbo::{Rect, Size};
use druid_shell::piet::Piet;
use druid_shell::MouseEvent;
use crate::app::event::{Event, EventResponse, Change};
use crate::app::widget_graph::WidgetContext;
use crate::app::size::PrefSize;

impl<T: Clone, W: Widget<T>, F: FnMut(Event, &mut W, WidgetContext, Key<T>) -> EventResponse> Widget<T> for EventModifier<W, F> {
    fn draw(&mut self, painter: &mut Piet, size: Size, dirty_rect: Rect, context: WidgetContext, data: &T) {
        self.0.draw(painter, size, dirty_rect, context, data)
    }

    fn handle_event(&mut self, event: Event, context: WidgetContext, data: Key<T>) -> EventResponse{
        self.1(event, &mut self.0, context, data)
    }

    fn get_pref_size(&mut self, context: WidgetContext, data: &T) -> PrefSize {
        self.0.get_pref_size(context, data)
    }

    fn layout(&mut self, size: Size, context: WidgetContext, data: &T) {
        self.0.layout(size, context, data)
    }

    fn build(&mut self, context: WidgetContext) {
        self.0.build(context)
    }

    fn update(&mut self, new: &T, old: Option<&T>) -> Change {
        self.0.update(new, old)
    }

    fn traverse_focus(&mut self, context: WidgetContext) -> bool {
        self.0.traverse_focus(context)
    }
}

pub struct EventModifier<W, F> (W, F);

pub fn click_listener<T: Clone, W: Widget<T>>(widget: W, mut listener: impl FnMut(MouseEvent, Key<T>)) -> impl Widget<T> {
    let mut pressed = false;
    event_listener(widget, move|event, data|{
        match event {
            Event::MouseDown(_) => {
                pressed = true;
                return EventResponse::DELETED;
            },
            Event::MouseExit => {
                pressed = false;
            },
            Event::MouseUp(event) => {
                if pressed {
                    pressed = false;
                    listener(event, data);
                    return EventResponse::DELETED;
                }
            },
            _ => {},
        }
        EventResponse::NONE
    })
}

pub fn event_listener<T: Clone, W: Widget<T>>(widget: W, mut listener: impl FnMut(Event, Key<T>) -> EventResponse) -> impl Widget<T> {
    EventModifier(widget, move|event: Event, widget: &mut W, context: WidgetContext, mut data: Key<T>|{
        let mut respone = widget.handle_event(event.clone(), context, data.id());
        if respone.is_present() {
            respone = respone.merge(listener(event, data))
        }
        respone
    })
}
pub fn event_filter<T: Clone, W: Widget<T>>(widget: W, mut listener: impl FnMut(Event, Key<T>) -> EventResponse) -> impl Widget<T> {
    EventModifier(widget, move|event: Event, widget: &mut W, context: WidgetContext, mut data: Key<T>|{
        let mut response = listener(event.clone(), data.id());
        if response.is_present() {
            response = response.merge(widget.handle_event(event, context, data))
        }
        response
    })
}

