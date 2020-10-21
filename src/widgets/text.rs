use crate::widgets::{Widget, PrefSize};
use druid_shell::kurbo::{Rect, Size};
use crate::widget_graph::WidgetContext;
use crate::event::{Event, EventResponse};
use druid_shell::piet::{Piet, RenderContext, PietTextLayout, FontBuilder, Text, TextLayout, Color, TextLayoutBuilder};
use crate::state::key::Key;

pub struct Label {
    text: String,
    layout: Option<PietTextLayout>,
}

impl Label {
    pub fn new(text: impl Into<String>) -> Self {
        Label {
            text: text.into(),
            layout: None,
        }
    }
}

impl<T: Clone> Widget<T> for Label {
    fn draw(&mut self, painter: &mut Piet, size: Size, dirty_rect: Rect, context: WidgetContext, data: &T) {
        if self.layout.is_none() {
            let font = painter.text().new_font_by_name("System", 15.0).build().unwrap();
            self.layout = Some(painter.text().new_text_layout(&font, &self.text, None).build().unwrap());
        }
        let brush = painter.solid_brush(Color::rgb8(255, 255, 255));
        painter.draw_text(self.layout.as_ref().unwrap(), (0.0, 20.0), &brush);
    }

    fn handle_event(&mut self, event: Event, context: WidgetContext, data: Key<T>) -> EventResponse{
        EventResponse::Valid
    }

    fn get_pref_size(&mut self, context: WidgetContext, data: &T) -> PrefSize {
        self.layout.as_ref().map_or(PrefSize::zero(), |layout|PrefSize::fixed((layout.width(), 25.0).into()))
    }

    fn layout(&mut self, size: Size, context: WidgetContext, data: &T) {

    }

    fn build(&mut self, context: WidgetContext) {

    }
}