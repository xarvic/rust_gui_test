mod hbox;
mod container;
mod const_contaner;
mod list_container;
mod vbox;
pub use hbox::HBox;
pub use vbox::VBox;
pub use container::Container;

use druid_shell::kurbo::{Size, Vec2};
use crate::widgets::PrefSize;

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

