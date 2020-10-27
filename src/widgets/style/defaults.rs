use once_cell::sync::Lazy;
use crate::widgets::style::{StyleAtlas, Style, Background, Fill};
use std::sync::Arc;
use std::collections::HashMap;
use druid_shell::piet::{StrokeStyle, Color};

static STYLES: Lazy<(HashMap<String, Arc<StyleAtlas>>, Arc<StyleAtlas>)> = Lazy::new(||{
    let default = Style{
        margin: 0.0,
        padding: 0.0,
        border: 0.0,
        border_style: StrokeStyle::new(),
        border_fill: Fill::Solid(Color::grey8(0)),
        background: None,
        corners: 0.0,
    };

    let default = Arc::new(StyleAtlas::new(default));

    let mut map = HashMap::new();

    let mut style = Style{
        margin: 0.0,
        padding: 4.0,
        border: 1.0,
        border_style: StrokeStyle::new(),
        border_fill: Fill::Solid(Color::grey8(100)),
        background: Some(Background::from_color(Color::grey8(80))),
        corners: 6.0,
    };

    let mut style_atlas = StyleAtlas::new(style.clone());

    style.background = Some(Background::from_color(Color::grey8(100)));

    style_atlas.hovered = Some(style.clone());

    style.background = None;
    style.border_fill = Fill::Solid(Color::grey8(200));

    style_atlas.pressed = Some(style.clone());

    map.insert("button".to_string(), Arc::new(style_atlas));

    style = Style{
        margin: 0.0,
        padding: 4.0,
        border: 1.0,
        border_style: StrokeStyle::new(),
        border_fill: Fill::Solid(Color::grey8(80)),
        background: Some(Background::from_color(Color::grey8(40))),
        corners: 6.0,
    };

    let mut style_atlas = StyleAtlas::new(style.clone());

    style.border = 2.0;
    style.border_fill = Fill::Solid(Color::rgb8(70, 80, 100));

    style_atlas.focused = Some(style.clone());

    style.background = None;
    style.border_fill = Fill::Solid(Color::grey8(200));

    style_atlas.pressed = Some(style.clone());

    map.insert("textfield".to_string(), Arc::new(style_atlas));

    (map, default)
});

pub fn get_style(name: &str) -> Arc<StyleAtlas> {
    STYLES.0.get(name).unwrap_or(&STYLES.1).clone()
}