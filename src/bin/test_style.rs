use gui::window::WindowBuilder;
use gui::widgets::layout::{Container, Spacing, HBox};
use gui::widgets::text::Label;
use gui::widgets::{WidgetCompose, Widget};
use gui::widgets::style::{StyleAtlas, Style, DynamicStyle, Background};
use std::sync::Arc;
use druid_shell::piet::{Brush, StrokeStyle, Color};
use gui::widgets::raw::{WrapperWidget, click_listener};
use gui::state::key::Key;
use druid_shell::MouseEvent;

fn button<T: Clone, F: FnMut(MouseEvent, Key<T>)>(style: &Arc<StyleAtlas>, text: impl Into<String>, handler: F) -> impl Widget<T> {
    click_listener(
        WrapperWidget::new(
            DynamicStyle::new(style.clone(), true),
            Label::new(text, None)
        ),
        handler
    )
}

fn main() {
    let mut style = Style{
        margin: 0.0,
        padding: 4.0,
        border: 2.0,
        border_style: StrokeStyle::new(),
        border_brush: Brush::Solid(Color::grey8(80).as_rgba_u32()),
        background: Some(Background::from_color(Color::grey8(70))),
        corners: 4.0
    };

    let mut style_atlas = StyleAtlas::new(style.clone());

    style.background = Some(Background::from_color(Color::grey8(80)));

    style_atlas.hovered = Some(style.clone());

    style.margin = 0.0;
    style.background = None;

    style_atlas.pressed = Some(style.clone());

    let style_atlas = Arc::new(style_atlas);

    WindowBuilder::new()
        .open(
            Container::new(HBox::new(Spacing::Right, 5.0))
                .child(Label::new("Button 1", None))
                .child(button(&style_atlas, "test 123", |_, mut key: Key<u32>|{
                    key.change(|value|*value += 1)
                }))
                .child(Label::new("Button 2", None))
                .child(button(&style_atlas, "ABC", |_, _|println!("Clicked 2!")))
                .background(Color::grey8(50), 10.0)
                .padding(20.0)
        )
}