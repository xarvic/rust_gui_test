use crate::widgets::Widget;
use druid_shell::kurbo::{Rect, Size};
use crate::widget_graph::WidgetContext;
use crate::event::{Event, EventResponse};
use druid_shell::piet::{Piet, RenderContext, PietTextLayout, FontBuilder, Text, TextLayout, Color, TextLayoutBuilder, PietText};
use crate::state::key::Key;
use crate::size::PrefSize;

pub struct Label {
    text: String,
    layout: PietTextLayout,
}

impl Label {
    pub fn new(text: impl Into<String>) -> Self {
        let text = text.into();
        let font = PietText::new().new_font_by_name("System", 15.0).build().unwrap();
        let layout = PietText::new().new_text_layout(&font, &text, None).build().unwrap();
        Label {
            text: text,
            layout,
        }
    }
}

impl<T: Clone> Widget<T> for Label {
    fn draw(&mut self, painter: &mut Piet, size: Size, dirty_rect: Rect, context: WidgetContext, data: &T) {
        let brush = painter.solid_brush(Color::rgb8(255, 255, 255));
        painter.draw_text(&self.layout, (0.0, 20.0), &brush);
    }

    fn handle_event(&mut self, event: Event, context: WidgetContext, data: Key<T>) -> EventResponse{
        EventResponse::NONE
    }

    fn get_pref_size(&mut self, context: WidgetContext, data: &T) -> PrefSize {
        PrefSize::fixed(Size::new(self.layout.width(), 25.0))
    }

    fn layout(&mut self, size: Size, context: WidgetContext, data: &T) {

    }

    fn build(&mut self, context: WidgetContext) {

    }
}