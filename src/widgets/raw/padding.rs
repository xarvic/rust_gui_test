use druid_shell::kurbo::{Size, Rect};
use crate::widgets::raw::{Wrapper, WrapperResponse};
use crate::event::Event;
use druid_shell::piet::Piet;

pub struct Padding(Size, Size);

impl Padding {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Padding(Size::new(x, y), Size::new(width, height))
    }
    pub fn equal(amount: f64) -> Self {
        Padding(Size::new(amount, amount), Size::new(amount, amount))
    }
    pub fn from_size(size: Size) -> Self{
        Padding(size, size)
    }
}

impl Wrapper for Padding {
    fn draw(&self, _piet: &mut Piet, _size: Size, _dirty_rect: Rect) {
    }

    fn handle_event(&mut self, _event: Event) -> WrapperResponse {
        WrapperResponse::Unchanged
    }

    fn get_left_top_padding(&self) -> Size {
        self.0
    }

    fn get_right_bottom_padding(&self) -> Size {
        self.1
    }
}

impl From<f64> for Padding {
    fn from(side: f64) -> Self {
        Padding::equal(side)
    }
}

impl From<(f64, f64)> for Padding {
    fn from(pad: (f64, f64)) -> Self {
        Padding::from_size(pad.into())
    }
}

impl From<Size> for Padding {
    fn from(size: Size) -> Self {
        Padding::from_size(size)
    }
}