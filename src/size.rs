use druid_shell::kurbo::{Size, Vec2};

pub static GROW_IF_NEEDED: f64 = 0.0000001;
pub static GROW_NORMAL: f64 = 1.0;
pub static DONT_GROW: f64 = 0.0;

///
#[derive(Copy, Clone, PartialEq)]
pub struct PrefSize {
    pub(crate) min: Size,
    pub(crate) max: Size,
    pub(crate) grow: Vec2,
}

impl PrefSize {
    pub fn fixed(size: impl Into<Size>) -> Self {
        let size = size.into().expand();
        PrefSize{
            min: size,
            max: size,
            grow: Vec2::ZERO
        }
    }
    pub fn min_max(min: impl Into<Size>, max: impl Into<Size>) -> Self {
        PrefSize::new(min, max, Vec2::new(DONT_GROW, DONT_GROW))
    }
    pub fn new(min: impl Into<Size>, max: impl Into<Size>, grow: impl Into<Vec2>) -> Self {
        let mut this = PrefSize {
            min: min.into().expand(),
            max: max.into().expand(),
            grow: grow.into().expand(),
        };
        this.max_max_size(this.min);
        this
    }

    pub fn flexible(size: impl Into<Size>) -> Self {
        Self::new(Size::ZERO, size, Vec2::new(GROW_IF_NEEDED, GROW_IF_NEEDED))
    }

    pub fn zero() -> Self {
        Self::fixed(Size::ZERO)
    }

    pub fn growing(mut self) -> Self {
        self.set_grow_y();
        self.set_grow_x();
        self
    }

    pub fn grow_if_needed(mut self) -> Self {
        self.grow_needed_y();
        self.grow_needed_x();
        self
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

    pub fn max_min_size(&mut self, size: Size) {
        self.min = Size::new(self.min.width.max(size.width), self.min.height.max(size.height))
    }
    pub fn min_min_size(&mut self, size: Size) {
        self.min = Size::new(self.min.width.min(size.width), self.min.height.min(size.height))
    }
    pub fn max_max_size(&mut self, size: Size) {
        self.max = Size::new(self.max.width.max(size.width), self.max.height.max(size.height))
    }
    pub fn min_max_size(&mut self, size: Size) {
        self.max = Size::new(self.max.width.min(size.width), self.max.height.min(size.height))
    }
    pub fn max_grow(&mut self, size: Vec2) {
        self.grow = Vec2::new(self.grow.x.max(size.x), self.grow.y.max(size.y))
    }
    pub fn min_grow(&mut self, size: Vec2) {
        self.grow = Vec2::new(self.grow.x.min(size.x), self.grow.y.min(size.y))
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