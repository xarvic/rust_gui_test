use crate::size::PrefSize;
use crate::widgets::layout::{Spacing, Layout, WidgetList};
use druid_shell::kurbo::{Size, Vec2};

pub struct HBox{
    inner: PrefSize,
    spacing: Spacing,
    children: u32,
    start_padding: f64,
    mid_padding: f64,
    end_padding: f64,
}

impl Default for HBox{
    fn default() -> Self {
        HBox{
            inner: PrefSize::zero(),
            spacing: Spacing::Equal,
            children: 0,
            start_padding: 10.0,
            mid_padding: 10.0,
            end_padding: 10.0,
        }
    }
}

impl HBox {
    pub fn new(spacing: Spacing, padding: f64) -> Self {
        HBox {
            inner: PrefSize::zero(),
            children: 0,
            spacing,
            start_padding: padding,
            mid_padding: padding,
            end_padding: padding,
        }
    }
    pub fn start_middle_end(spacing: Spacing, start: f64, middle: f64, end: f64) -> Self {
        HBox {
            inner: PrefSize::zero(),
            children: 0,
            spacing,
            start_padding: start,
            mid_padding: middle,
            end_padding: end,
        }
    }
    fn with_const_padding(&self) -> PrefSize {
        let const_padding = self.const_padding();
        let mut pref_size = self.inner;
        pref_size.min.width += const_padding;
        pref_size.max.width += const_padding;
        pref_size.set_grow_x();
        pref_size
    }
    fn const_padding(&self) -> f64 {
        if self.children > 0 {
            self.start_padding + self.mid_padding * ((self.children - 1) as f64) + self.end_padding
        } else {
            0.0
        }
    }
}

pub struct Index(usize);

impl Default for Index {
    fn default() -> Self {
        Index(usize::max_value())
    }
}

impl Layout for HBox {
    type Constrain = Index;
    type Meta = ();
    const CAN_OVERLAP: bool = false;

    fn insert(&mut self, constrain: Self::Constrain) -> (u32, Self::Meta) {
        let index = (constrain.0 as u32).min(self.children);
        self.children += 1;
        (index, ())
    }

    fn remove(&mut self, index: u32) {
        self.children -= 1;
    }

    fn clear(&mut self) {
        self.children = 0;
    }

    fn overlaping(&self) -> bool {
        false
    }

    fn calc_pref_size(&mut self, widgets: &impl WidgetList<Self::Meta>) -> PrefSize {
        self.inner = PrefSize::zero();
        self.children = widgets.count();
        widgets.iter_inner(|meta|{
            self.inner.row(meta.pref);
        });
        self.with_const_padding()
    }

    fn get_pref_size(&self) -> PrefSize {
        self.with_const_padding()
    }


    fn layout(&mut self, mut size: Size, widgets: &mut impl WidgetList<Self::Meta>) {

        size.width -= self.const_padding();

        let pref_size = self.inner;

        let variance = pref_size.max.width - pref_size.min.width;

        // A number between 0 and 1 to determine how much space is available
        // 0: min_size of less available => widgets will take the min_size
        // 1: max_size of more available => widgets will take the max_size
        // otherwise use the value to interpolate between min and max value
        let rel = if variance > 0.0 {
            (size.width
                .max(pref_size.min.width)
                .min(pref_size.max.width)
                - pref_size.min.width
            ) / variance
        } else {
            1.0
        };

        let remaining = size.width
                .max(pref_size.max.width)
                - pref_size.max.width;

        let mut add = 0.0;

        let (mut padding, mut next_x) = if pref_size.grow.x > 0.0 {
            add = remaining / pref_size.grow.x;
            (0.0, 0.0)
        } else {
            match self.spacing {
                Spacing::Between => (remaining / (self.children - 1) as f64, 0.0),
                Spacing::Padding => (remaining / (self.children as f64), remaining / (self.children as f64) / 2.0),
                Spacing::Around => (0.0, remaining / 2.0),
                Spacing::Equal => (remaining / (self.children + 1) as f64, remaining / (self.children + 1) as f64),
                Spacing::Left => (0.0, remaining),
                Spacing::Right => (0.0, 0.0),
            }
        };

        padding += self.mid_padding;
        next_x += self.start_padding;


        widgets.iter_inner_mut(|child| {
            let child_pref = child.pref;
            let child_size =
                Size::new(child_pref.min.width * (1.0 - rel) +
                              child_pref.max.width * rel +
                              child_pref.grow.x * add,
                          if child_pref.grow.y != 0.0 {
                                    size.height
                                        .max(child_pref.min.height)
                                } else {
                                    size.height
                                        .min(child_pref.max.height)
                                        .max(child_pref.min.height)
                                }
                );

            child.size = child_size;
            child.offset = Vec2::new(next_x, (size.height - child_size.height) / 2.0);
            next_x += child_size.width + padding;
        });
    }
}