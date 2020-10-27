use crate::widgets::text::Label;
use crate::widgets::Widget;
use druid_shell::kurbo::{Rect, Size};
use druid_shell::piet::Piet;
use crate::state::key::Key;
use druid_shell::KeyCode;
use crate::app::widget_graph::WidgetContext;
use crate::app::event::{EventResponse, Event, Change};
use crate::app::size::PrefSize;

pub struct TextInput{
    text: Label,
    //Measured in chars
    cursor_position: usize,
    focused: bool,
}

impl TextInput {
    pub fn new() -> Self {
        TextInput {
            text: Label::new("", None),
            cursor_position: 0,
            focused: false,
        }
    }
}

impl Widget<String> for TextInput {
    fn draw(&mut self, painter: &mut Piet, size: Size, dirty_rect: Rect, context: WidgetContext, data: &String) {
        self.text.draw(painter, size, dirty_rect, context, data)
    }

    fn handle_event(&mut self, event: Event, context: WidgetContext, mut data: Key<String>) -> EventResponse {
        if let Event::KeyDown(key_event) = event {
            match key_event.key_code {
                KeyCode::Backspace => {
                    if data.len() > 0 {
                        data.change(|value|{
                            let last_index = value.chars().count() - 1;
                            value.remove(last_index)
                        });
                    }
                }
                _ => {
                    if let Some(text) = key_event.text() {
                        data.change(|value|value.push_str(text));
                    }
                }
            }
        }

        EventResponse::NONE
    }

    fn get_pref_size(&mut self, context: WidgetContext, data: &String) -> PrefSize {
        PrefSize::min_max((80.0, 17.0), (150.0, 17.0))
    }

    fn layout(&mut self, size: Size, context: WidgetContext, data: &String) {

    }

    fn build(&mut self, context: WidgetContext) {

    }

    fn update(&mut self, new: &String, old: Option<&String>) -> Change {
        self.text.set_text().clone_from(new);
        self.text.recalc_text();
        Change::Bounds
    }
    fn traverse_focus(&mut self, context: WidgetContext) -> bool {
        self.focused = !self.focused;
        println!("{}focused textfield!", if self.focused {""} else {"un"});
        self.focused
    }
}