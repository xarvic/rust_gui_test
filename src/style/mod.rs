use druid_shell::piet::{StrokeStyle, Brush, Piet};
use druid_shell::kurbo::Size;
use std::sync::Arc;

struct Background {
    brush: Brush,
}



struct Style {
    border: f64,
    border_style: StrokeStyle,
    border_brush: Brush,

    background: Option<Background>,

    corners: [f64; 4],
}

impl Style {
    pub fn draw(&self, piet: &mut Piet, size: Size) {

    }
}


struct StyleAtlas {
    idle: Style,
    focused: Style,
    hovered: Style,
    hovered_focused: Style,
    clicked: Style,
    active: Style,
}

enum MouseState {
    None,
    Hovered,
    Clicked,
}

struct DynamicStyle {
    atlas: Arc<StyleAtlas>,
    focused: bool,
    active: bool,
    mouse_state: MouseState,
}