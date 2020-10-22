mod hbox;
mod container;
mod const_contaner;
mod list_container;
mod vbox;
pub use hbox::HBox;
pub use vbox::VBox;
pub use container::Container;

use druid_shell::kurbo::{Size, Vec2, Rect, Affine};
use crate::widgets::Widget;
use crate::widget_graph::WidgetContext;
use crate::event::{Event, EventResponse};
use druid_shell::piet::{Piet, RenderContext};
use crate::state::key::Key;

pub struct ChildMeta<Meta> {
    pub meta: Meta,
    pub offset: Vec2,
    pub size: Size,
    pub pref: PrefSize,
}

impl<Meta> ChildMeta<Meta> {
    pub fn empty(meta: Meta) -> Self {
        ChildMeta {
            meta,
            offset: Vec2::ZERO,
            size: Size::ZERO,
            pref: PrefSize::zero(),
        }
    }
    pub fn new(meta: Meta, offset: Vec2, size: Size, pref_size: PrefSize) -> Self {
        ChildMeta {
            meta,
            offset,
            size,
            pref: pref_size,
        }
    }
}

pub trait WidgetList<Meta> {
    fn iter_inner(&self, iterator: impl FnMut(&ChildMeta<Meta>));
    fn iter_inner_mut(&mut self, iterator: impl FnMut(&mut ChildMeta<Meta>));
    fn get_with(&self, index: usize, iterator: impl FnMut(&ChildMeta<Meta>));
    fn get_mut_with(&mut self, index: usize, iterator: impl FnMut(&mut ChildMeta<Meta>));
    fn count(&self) -> u32;
}

/// The Layout of a Container
/// the display order is back to front
pub trait Layout {
    /// Constrain is the
    type Constrain;
    type Meta;

    ///
    const CAN_OVERLAP: bool = true;


    fn insert(&mut self, constrain: Self::Constrain) -> (u32, Self::Meta);
    fn remove(&mut self, index: u32);
    fn clear(&mut self);

    fn overlaping(&self) -> bool;
    fn calc_pref_size(&mut self, widgets: &impl WidgetList<Self::Meta>) -> PrefSize;
    fn get_pref_size(&self) -> PrefSize;
    fn layout(&mut self, size: Size, widgets: &mut impl WidgetList<Self::Meta>);

}

pub trait ListLayout: Layout + Default {
    fn next(&mut self) -> Self::Meta;
}

/// Defines how a container should place its children if there is additional space but
/// none of them wants to grow.
/// If there is only one child, all options except Left and Right behave the same
pub enum Spacing {
    /// Creates space between the Child Widgets, but not at the End
    ///
    /// (| W1 |--------|  W2  |--------|W3|--------| W4 |)
    ///
    Between,
    /// Creates space at the Left and Right End, but not between the Children
    ///
    /// (---| W1 |------|  W2  |------|W3|------| W4 |---)
    ///
    Padding,
    /// creates equaly lange space around each Child
    ///
    /// (------------| W1 ||  W2  ||W3|| W4 |------------)
    ///
    Around,
    ///
    ///
    /// (-----| W1 |-----|  W2  |-----|W3|-----| W4 |-----)
    ///
    Equal,
    /// Creates all space at the left end
    ///
    /// (------------------------| W1 ||  W2  ||W3|| W4 |)
    ///
    Left,
    /// Creates all space at the right end
    ///
    /// (| W1 ||  W2  ||W3|| W4 |------------------------)
    ///
    Right,
}

pub static GROW_IF_NEEDED: f64 = 0.0000001;
pub static GROW_NORMAL: f64 = 1.0;
pub static DONT_GROW: f64 = 0.0;

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

    pub fn stacked(self, other: Self) -> Self {
        PrefSize {
            min: Size::new(self.min.width.max(other.min.width), self.min.height.max(other.min.height)),
            max: Size::new(self.max.width.max(other.max.width), self.max.height.max(other.max.height)),
            grow: Vec2::new(self.grow.x.max(other.grow.x), self.grow.y.max(other.grow.y))
        }
    }

    pub fn resize(&mut self, size: impl Into<Size>) {
        let size = size.into();
        self.min += size;
        self.max += size;
    }

    pub fn set_grow_x(&mut self) {
        self.grow.x = self.grow.x.max(GROW_NORMAL);
    }
    pub fn set_grow_y(&mut self) {
        self.grow.y = self.grow.y.max(GROW_NORMAL);
    }
    pub fn grow_needed_x(&mut self) {
        self.grow.x = self.grow.x.max(GROW_IF_NEEDED);
    }
    pub fn grow_needed_y(&mut self) {
        self.grow.y = self.grow.y.max(GROW_IF_NEEDED);
    }
    pub fn dont_grow_x(&mut self) {
        self.grow.x = DONT_GROW
    }
    pub fn dont_grow_y(&mut self) {
        self.grow.y = DONT_GROW
    }
}

pub struct Padding<W>(W, Size, Size);

impl<T: Clone, W: Widget<T>> Widget<T> for Padding<W> {
    fn draw(&mut self, painter: &mut Piet, size: Size, dirty_rect: Rect, context: WidgetContext, data: &T) {
        painter.transform(Affine::translate(self.1.to_vec2()));
        self.0.draw(painter, size - self.2, dirty_rect.inflate(-self.1.width, -self.1.height), context, data);
        painter.transform(Affine::translate(-self.1.to_vec2()));
    }

    fn handle_event(&mut self, mut event: Event, context: WidgetContext, data: Key<T>) -> EventResponse{
        if event.shift(self.1.to_vec2(), Size::from((f64::INFINITY, f64::INFINITY))) {//TODO: fix me size is unknown!
            self.0.handle_event(event, context, data).shift(self.1.to_vec2())
        } else {
            EventResponse::NONE
        }
    }

    fn get_pref_size(&mut self, context: WidgetContext, data: &T) -> PrefSize {
        let mut pref = self.0.get_pref_size(context, data);
        pref.resize(self.2);
        pref
    }

    fn layout(&mut self, size: Size, context: WidgetContext, data: &T) {
        self.0.layout(size - self.2, context, data)
    }

    fn build(&mut self, context: WidgetContext) {
        self.0.build(context)
    }
    fn traverse_focus(&mut self, context: WidgetContext) -> bool {
        self.0.traverse_focus(context)
    }
}

pub fn padding<W>(size: f64, widget: W) -> Padding<W> {
    Padding(widget, Size::new(size, size), Size::new(size * 2.0, size * 2.0))
}