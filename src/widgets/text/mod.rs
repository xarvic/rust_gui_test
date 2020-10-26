use crate::widgets::Widget;
use druid_shell::kurbo::{Rect, Size};
use crate::widget_graph::WidgetContext;
use crate::event::{Event, EventResponse};
use druid_shell::piet::{Piet, RenderContext, PietTextLayout, FontBuilder, Text, TextLayout, Color, TextLayoutBuilder, PietText, PietFont};
use crate::state::key::Key;
use crate::size::PrefSize;
use crate::widgets::widget::IntoWidget;
use std::marker::PhantomData;

impl<T: Clone> IntoWidget<T> for String {
    type Widget = Label;

    fn into_widget(self) -> Self::Widget {
        Label::new(self, None)
    }
}

impl<T: Clone> IntoWidget<T> for &str {
    type Widget = Label;

    fn into_widget(self) -> Self::Widget {
        Label::new(self.to_string(), None)
    }
}

pub struct Label {
    text: String,
    font: PietFont,
    layout: PietTextLayout,
}

impl Label {
    pub fn new<'a>(text: impl Into<String>, font: impl Into<Option<&'a str>>) -> Self {
        let text = text.into();
        let font = PietText::new().new_font_by_name(font.into().unwrap_or("System"), 13.0).build().unwrap();
        let layout = PietText::new().new_text_layout(&font, &text, None).build().unwrap();
        Label {
            text,
            font,
            layout,
        }
    }
    pub fn set_text(&mut self) -> &mut String {
        &mut self.text
    }
    pub fn recalc_text(&mut self) {
        self.layout = PietText::new().new_text_layout(&self.font, &self.text, None).build().unwrap();
    }
}

impl<T: Clone> Widget<T> for Label {
    fn draw(&mut self, painter: &mut Piet, size: Size, dirty_rect: Rect, context: WidgetContext, data: &T) {
        let brush = painter.solid_brush(Color::rgb8(255, 255, 255));
        painter.draw_text(&self.layout, (4.0, 13.0), &brush);
    }

    fn handle_event(&mut self, event: Event, context: WidgetContext, data: Key<T>) -> EventResponse{
        EventResponse::NONE
    }

    fn get_pref_size(&mut self, context: WidgetContext, data: &T) -> PrefSize {
        PrefSize::fixed(Size::new(self.layout.width() + 8.0, 17.0))
    }

    fn layout(&mut self, size: Size, context: WidgetContext, data: &T) {

    }

    fn build(&mut self, context: WidgetContext) {

    }
}

pub struct DynLabel<T, F> {
    label: Label,
    update: F,
    size: Size,
    phantop: PhantomData<T>,
}

impl<T: Clone, F: Fn(&T) -> String> DynLabel<T, F> {
    pub fn new(update: F) -> Self {
        DynLabel{
            label: Label::new("", None),
            update,
            size: Size::ZERO,
            phantop: PhantomData,
        }
    }
}

impl<T: Clone, F: Fn(&T) -> String> Widget<T> for DynLabel<T, F> {
    fn draw(&mut self, painter: &mut Piet, size: Size, dirty_rect: Rect, context: WidgetContext, data: &T) {
        self.label.draw(painter, size, dirty_rect, context, data)
    }

    fn handle_event(&mut self, event: Event, context: WidgetContext, data: Key<T>) -> EventResponse {
        //TODO: move this to update
        *self.label.set_text() = (self.update)(&*data);
        self.label.recalc_text();

        EventResponse::bounds_changed(false)
    }

    fn get_pref_size(&mut self, context: WidgetContext, data: &T) -> PrefSize {
        *self.label.set_text() = (self.update)(data);
        self.label.recalc_text();

        self.label.get_pref_size(context, data)
    }

    fn layout(&mut self, size: Size, context: WidgetContext, data: &T) {
        self.size = size;
    }

    fn build(&mut self, context: WidgetContext) {}
}