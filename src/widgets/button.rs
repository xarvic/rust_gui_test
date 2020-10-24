use crate::widgets::style::{StyleAtlas, DynamicStyle};
use std::sync::Arc;
use druid_shell::MouseEvent;
use crate::state::key::Key;
use crate::widgets::{Widget, WidgetCompose, IntoWidget};
use crate::widgets::raw::click_listener;

pub fn button<T: Clone>(widget: impl IntoWidget<T>, style: Arc<StyleAtlas>, handler: impl FnMut(MouseEvent, Key<T>)) -> impl Widget<T> {
    click_listener(
        widget.wrap(DynamicStyle::new(style, true)),
        handler
    )
}

pub fn button_group<T: Clone + PartialEq>(my_value: T, widget: impl IntoWidget<T>, style: Arc<StyleAtlas>) -> impl Widget<T> {
    unimplemented!()
}

pub fn toggle_button(widget: impl IntoWidget<bool>, style: Arc<StyleAtlas>) -> impl Widget<bool> {
    unimplemented!()
}