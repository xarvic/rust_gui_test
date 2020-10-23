use crate::widgets::raw::{Wrapper, WrapperResponse};
use crate::widgets::style::Style;
use druid_shell::piet::Piet;
use druid_shell::kurbo::{Size, Rect};
use crate::event::Event;

pub struct StaticStyle(Style);

impl Wrapper for StaticStyle {
    fn draw(&self, piet: &mut Piet, size: Size, dirty_rect: Rect) {
        self.0.draw(piet, size)
    }

    fn handle_event(&mut self, event: Event) -> WrapperResponse {
        WrapperResponse::Unchanged
    }

    fn get_left_top_padding(&self) -> Size {
        Size::new(self.0.full_padding(), self.0.full_padding())
    }

    fn get_right_bottom_padding(&self) -> Size {
        Size::new(self.0.full_padding(), self.0.full_padding())
    }
}