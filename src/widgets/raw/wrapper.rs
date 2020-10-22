use druid_shell::piet::{Piet, RenderContext};
use druid_shell::kurbo::{Size, Rect, Affine};
use crate::event::{Event, EventResponse};
use crate::widgets::Widget;
use crate::widget_graph::WidgetContext;
use crate::state::key::Key;
use crate::size::PrefSize;

pub enum WrapperResponse {
    Unchanged,
    Changed,
    Deleted,
    Consumed,
}

impl WrapperResponse {
    pub fn content_change(&self) -> bool {
        if let WrapperResponse::Changed | WrapperResponse::Consumed = self {
            true
        } else {
            false
        }
    }
    pub fn consumed(&self) -> bool {
        if let WrapperResponse::Deleted | WrapperResponse::Consumed = self {
            true
        } else {
            false
        }
    }
    pub fn to_event_response(&self, content_size: Size) -> EventResponse {
        if self.content_change() {
            EventResponse::changed(content_size, self.consumed())
        } else {
            EventResponse::unchanged(self.consumed())
        }
    }
}

pub trait Wrapper {
    /// Draws the Wrapper
    /// The Wrapper will always draw before the wrapped Component
    fn draw(&self, piet: &mut Piet, size: Size, dirty_rect: Rect);

    /// Handles a Event originated for the Wrapped Component
    /// This method will only get called if the EventResponse of the wrapped Widget didnt consumed
    /// the Event
    ///
    /// If because of the Event the content changes, the WrapperRespone must contain a content_change
    /// The Wrapper-widget will detect size changes independently
    fn handle_event(&mut self, event: Event) -> WrapperResponse;

    /// Returns the TopLeft Padding of the Widget
    ///
    /// The Wrapper can draw to the full size of the Wrapping Widget, but the wrapped Widget, will
    /// get shifted by this value
    fn get_left_top_padding(&self) -> Size;

    /// Returns the BottomRight Padding of the Widget
    fn get_right_bottom_padding(&self) -> Size;

    fn get_full_padding(&self) -> Size {
        self.get_left_top_padding() + self.get_right_bottom_padding()
    }
}

pub struct WrapperWidget<U, W> {
    wrapped_widget: W,
    wrapper: U,
    shift: Size,
    padding: Size,
    content_size: Size,
}

impl<U: Wrapper, W> WrapperWidget<U, W> {
    pub fn new(wrapper: U, wrapped_widget: W) -> Self {
        let shift = wrapper.get_left_top_padding();
        let padding = shift + wrapper.get_right_bottom_padding();

        WrapperWidget {
            wrapped_widget,
            wrapper,
            shift,
            padding,
            content_size: Size::ZERO,
        }
    }
}

impl<T: Clone, U: Wrapper, W: Widget<T>> Widget<T> for WrapperWidget<U, W> {
    fn draw(&mut self, painter: &mut Piet, mut size: Size, dirty_rect: Rect, context: WidgetContext, data: &T) {
        let _ = painter.with_save(|painter|{
            self.wrapper.draw(painter, size, dirty_rect);
            Ok(())
        });
        size = size - self.padding;
        let child_dirty_rect = Rect::new(dirty_rect.x0 - self.shift.width,
                                         dirty_rect.y0 - self.shift.height,
                                         dirty_rect.x1 - self.shift.width,
                                         dirty_rect.y1 - self.shift.height)
            .intersect(size.to_rect());

        painter.transform(Affine::translate(self.shift.to_vec2()));
        self.wrapped_widget.draw(painter, size, child_dirty_rect, context, data);
    }

    fn handle_event(&mut self, event: Event, context: WidgetContext, data: Key<T>) -> EventResponse{
        let mut client_event = event.clone();
        let mut response = if client_event.shift(self.shift.to_vec2(), self.content_size) {
            self.wrapped_widget.handle_event(client_event, context, data).shift(self.shift.to_vec2())
        } else {
            EventResponse::unchanged(false)
        };
        if response.is_present() {
            let response2 = self.wrapper.handle_event(event);
            if self.shift == self.wrapper.get_left_top_padding() &&
                self.padding == self.wrapper.get_full_padding() {
                response = response.merge(response2.to_event_response(self.content_size + self.padding));
            } else {
                self.shift = self.wrapper.get_left_top_padding();
                self.padding = self.wrapper.get_full_padding();
                response = response.merge(EventResponse::bounds_changed(response2.consumed()));
            }
        }
        response
    }

    fn get_pref_size(&mut self, context: WidgetContext, data: &T) -> PrefSize {
        let mut pref = self.wrapped_widget.get_pref_size(context, data);
        pref.resize(self.padding);
        pref
    }

    fn layout(&mut self, size: Size, context: WidgetContext, data: &T) {
        self.content_size = size - self.padding;
        self.wrapped_widget.layout(self.content_size, context, data)
    }

    fn build(&mut self, context: WidgetContext) {
        self.wrapped_widget.build(context)
    }
    fn traverse_focus(&mut self, context: WidgetContext) -> bool {
        self.wrapped_widget.traverse_focus(context)
    }
}