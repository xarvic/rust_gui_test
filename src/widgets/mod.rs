use druid_shell::piet::Piet;
use crate::event::Event;
use crate::state::key::Key;
use druid_shell::kurbo::{Size, Rect};
use crate::widget_graph::WidgetContext;

pub mod layout;
pub mod state;
pub mod lens;


pub trait Widget<T: Clone> {
    fn draw(&self, painter: &mut Piet, dirty_rect: Rect, context: WidgetContext, data: &T);

    fn handle_event(&mut self, event: Event, context: WidgetContext, data: Key<T>);

    fn get_pref_size(&self, context: WidgetContext, data: &T) -> PrefSize;

    fn layout(&mut self, size: Size, context: WidgetContext, data: &T);

    fn build(&mut self, context: WidgetContext);

    #[allow(unused_variables)]
    fn traverse_focus(&mut self, context: WidgetContext) -> bool {false}
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
pub struct Empty;

impl<T: Clone> Widget<T> for Empty{
    fn draw(&self, painter: &mut Piet, dirty_rect: Rect, context: WidgetContext, data: &T) {}

    fn handle_event(&mut self, event: Event, context: WidgetContext, data: Key<T>) {}

    fn get_pref_size(&self, context: WidgetContext, data: &T) -> PrefSize { PrefSize::fixed(Size::ZERO) }

    fn layout(&mut self, size: Size, context: WidgetContext, data: &T) {}

    fn build(&mut self, context: WidgetContext) {}

    fn traverse_focus(&mut self, context: WidgetContext) -> bool { false }
}

#[derive(Copy, Clone, PartialEq, Hash)]
pub struct PrefSize {
    min: Size,
    max: Size,
}

impl PrefSize {
    pub fn fixed(size: Size) -> Self {
        PrefSize{
            min: size,
            max: size,
        }
    }
    pub fn new(min: Size, max: Size) -> Self {
        PrefSize{
            min,
            max,
        }
    }
    pub fn zero() -> Self {
        Self::fixed(Size::ZERO)
    }

    pub fn row(&mut self, other: Self) {
        self.min.width += other.min.width;
        self.min.height = self.min.height.max(other.min.height);
        self.max.width += other.max.width;
        self.max.height = self.max.height.max(other.max.height);
    }
    pub fn column(&mut self, other: Self) {
        self.min.width = self.min.width.max(other.min.width);
        self.min.height += other.min.height;
        self.max.width = self.max.width.max(other.max.height);
        self.max.height += other.max.height;
    }
}


