use crate::widgets::style::{StyleAtlas, DynamicStyle, get_style};
use std::sync::Arc;
use druid_shell::MouseEvent;
use crate::state::key::Key;
use crate::widgets::{Widget, WidgetCompose};
use crate::widgets::raw::click_listener;

pub fn button<T: Clone>(widget: impl Widget<T>, style: impl Into<Option<Arc<StyleAtlas>>>, handler: impl FnMut(MouseEvent, Key<T>)) -> impl Widget<T> {
    click_listener(
        widget.wrap(DynamicStyle::new(style.into().unwrap_or(get_style("button")), true)),
        handler
    )
}
