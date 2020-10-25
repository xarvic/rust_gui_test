mod background;
mod static_style;
mod dynamic_style;
mod defaults;

pub use background::{background, BackgroundWrapper};
pub use dynamic_style::{MouseState, StyleAtlas, DynamicStyle};
pub use static_style::{StaticStyle};
pub use defaults::get_style;

use druid_shell::piet::{StrokeStyle, Piet, Color, RenderContext};
use druid_shell::kurbo::{Size, Shape, Rect};

#[derive(Clone)]
pub enum Fill {
    Solid(Color),
}

impl PartialEq for Fill {
    fn eq(&self, other: &Self) -> bool {
        let Fill::Solid(Color::Rgba32(value)) = self;
        let Fill::Solid(Color::Rgba32(value2)) = other;
        value == value2
    }
}

#[derive(Clone, PartialEq)]
pub struct Background {
    color: Fill,
    corner_radius: f64,
}

impl Background {
    pub fn from_color(color: impl Into<Color>) -> Self{
        Background {
            color: Fill::Solid(color.into()),
            corner_radius: 0.0,
        }
    }

    pub fn draw(&self, piet: &mut Piet, shape: impl Shape) {
        let Fill::Solid(color) = &self.color;
        let brush = piet.solid_brush(color.clone());
        piet.fill(shape, &brush);
    }
}


#[derive(Clone, PartialEq)]
pub struct Style {
    pub margin: f64,
    pub padding: f64,

    pub border: f64,
    pub border_style: StrokeStyle,
    pub border_fill: Fill,

    pub background: Option<Background>,

    pub corners: f64,
}

impl Style {
    pub fn draw(&self, piet: &mut Piet, size: Size) {
        if let Some(ref background) = self.background {
            let back = Rect::new(self.margin, self.margin, size.width - self.margin, size.height - self.margin);
            if self.corners < 1.0 {
                background.draw(piet, back);
            } else {
                background.draw(piet, back.to_rounded_rect(self.corners));
            }
        }
        if self.border >= 1.0 {
            let center = self.margin + self.border / 2.0;
            let border = Rect::new(center, center, size.width - center, size.height - center);
            let Fill::Solid(color) = &self.border_fill;
            let brush = piet.solid_brush(color.clone());

            if self.corners <= 1.0 {
                piet.stroke_styled(border, &brush, self.border, &self.border_style);
            } else {
                piet.stroke_styled(border.to_rounded_rect(self.corners), &brush, self.border, &self.border_style);
            }
        }
    }
    pub fn full_padding(&self) -> f64 {
        self.padding + self.border + self.margin
    }
}


