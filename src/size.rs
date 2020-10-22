use druid_shell::kurbo::{Size, Vec2};

pub static GROW_IF_NEEDED: f64 = 0.0000001;
pub static GROW_NORMAL: f64 = 1.0;
pub static DONT_GROW: f64 = 0.0;

#[derive(Copy, Clone, PartialEq)]
pub struct PrefSize {
    pub(crate) min: Size,
    pub(crate) max: Size,
    pub(crate) grow: Vec2,
}

impl PrefSize {
    pub fn fixed(size: Size) -> Self {
        PrefSize{
            min: size.expand(),
            max: size.expand(),
            grow: Vec2::ZERO
        }
    }
    pub fn min_max(min: Size, max: Size) -> Self {
        PrefSize{
            min: min.expand(),
            max: max.expand(),
            grow: Vec2::ZERO
        }
    }
    pub fn new(min: Size, max: Size, grow: Vec2) -> Self {
        PrefSize {
            min: min.expand(),
            max: max.expand(),
            grow: grow.expand(),
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
        let size = size.into().expand();
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

