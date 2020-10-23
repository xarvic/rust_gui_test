mod background;
mod static_style;
mod dynamic_style;

pub use background::{background, BackgroundWrapper};
pub use dynamic_style::{MouseState, StyleAtlas, DynamicStyle};
pub use static_style::{StaticStyle};

use druid_shell::piet::{StrokeStyle, Brush, Piet, Color, RenderContext};
use druid_shell::kurbo::{Size, Shape, Rect};

#[derive(Clone)]
pub struct Background {
    color: Color,
    corner_radius: f64,
}

impl Background {
    pub fn from_color(color: impl Into<Color>) -> Self{
        Background {
            color: color.into(),
            corner_radius: 0.0,
        }
    }

    pub fn draw(&self, piet: &mut Piet, shape: impl Shape) {
        let brush = piet.solid_brush(self.color.clone());
        piet.fill(shape, &brush);
    }
}


#[derive(Clone)]
pub struct Style {
    pub margin: f64,
    pub padding: f64,

    pub border: f64,
    pub border_style: StrokeStyle,
    pub border_brush: Brush,

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

            if self.corners <= 1.0 {
                piet.stroke_styled(border, &self.border_brush, self.border, &self.border_style);
            } else {
                piet.stroke_styled(border.to_rounded_rect(self.corners), &self.border_brush, self.border, &self.border_style);
            }
        }
    }
    pub fn full_padding(&self) -> f64 {
        self.padding + self.border + self.margin
    }
}


