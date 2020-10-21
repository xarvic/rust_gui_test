use druid_shell::piet::Piet;
use crate::event::Event;
use crate::state::key::Key;
use druid_shell::kurbo::{Size, Rect, Vec2};
use crate::widget_graph::WidgetContext;

pub mod layout;
pub mod state;
pub mod lens;

mod text;

pub use text::Label;

pub trait Widget<T: Clone> {
    fn draw(&mut self, painter: &mut Piet, size: Size, dirty_rect: Rect, context: WidgetContext, data: &T);

    fn handle_event(&mut self, event: Event, context: WidgetContext, data: Key<T>);

    fn get_pref_size(&mut self, context: WidgetContext, data: &T) -> PrefSize;

    fn layout(&mut self, size: Size, context: WidgetContext, data: &T);

    fn build(&mut self, context: WidgetContext);

    #[allow(unused_variables)]
    fn traverse_focus(&mut self, context: WidgetContext) -> bool {false}
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
pub struct Empty;

impl<T: Clone> Widget<T> for Empty{
    fn draw(&mut self, painter: &mut Piet, size: Size, dirty_rect: Rect, context: WidgetContext, data: &T) {}

    fn handle_event(&mut self, event: Event, context: WidgetContext, data: Key<T>) {}

    fn get_pref_size(&mut self, context: WidgetContext, data: &T) -> PrefSize { PrefSize::fixed(Size::ZERO) }

    fn layout(&mut self, size: Size, context: WidgetContext, data: &T) {}

    fn build(&mut self, context: WidgetContext) {}

    fn traverse_focus(&mut self, context: WidgetContext) -> bool { false }
}

#[derive(Copy, Clone, PartialEq)]
pub struct PrefSize {
    min: Size,
    max: Size,
    grow: Vec2,
}

impl PrefSize {
    pub fn fixed(size: Size) -> Self {
        PrefSize{
            min: size,
            max: size,
            grow: Vec2::ZERO
        }
    }
    pub fn min_max(min: Size, max: Size) -> Self {
        PrefSize{
            min,
            max,
            grow: Vec2::ZERO
        }
    }
    pub fn new(min: Size, max: Size, grow: Vec2) -> Self {
        PrefSize {
            min,
            max,
            grow,
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
        self.grow.x += other.grow.x;
        self.grow.y += self.grow.y.max(other.grow.y);
    }
    pub fn column(&mut self, other: Self) {
        self.min.width = self.min.width.max(other.min.width);
        self.min.height += other.min.height;
        self.max.width = self.max.width.max(other.max.height);
        self.max.height += other.max.height;
        self.grow.x += self.grow.x.max(other.grow.x);
        self.grow.y += other.grow.y;
    }
    pub fn set_grow_x(&mut self) {
        self.grow.x = self.grow.x.max(1.0);
    }
    pub fn set_grow_y(&mut self) {
        self.grow.y = self.grow.y.max(1.0);
    }
}


