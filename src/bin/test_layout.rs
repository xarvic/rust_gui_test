use gui::window::WindowBuilder;
use gui::widgets::{Widget, WidgetCompose};
use druid_shell::piet::{Color, Piet, RenderContext};
use druid_shell::kurbo::{Rect, Size};
use gui::widget_graph::WidgetContext;
use gui::event::{Event, EventResponse};
use gui::state::key::Key;
use gui::widgets::layout::{HBox, Container, Spacing, VBox};
use gui::size::PrefSize;
use gui::widgets::text::Label;

struct ColorRect(Color, bool, Size, u32);

impl ColorRect {
    pub fn new(color: Color, id: u32) -> Self {
        ColorRect(color, false, Size::ZERO, id)
    }
}

impl Widget<u32> for ColorRect {
    fn draw(&mut self, painter: &mut Piet, size: Size, _dirty_rect: Rect, _context: WidgetContext, _data: &u32) {
        let brush = painter.solid_brush(if self.1 {Color::rgb8(100, 100, 255)} else {self.0.clone()});
        painter.stroke(Rect::new(5.0, 5.0, size.width - 5.0, size.height - 5.0), &brush, 10.0);
    }

    fn handle_event(&mut self, event: Event, _context: WidgetContext, _data: Key<u32>) -> EventResponse{
        match event {
            Event::MouseDown(_) => {
                if !self.1 {
                    self.1 = true;
                    EventResponse::changed(self.2, true)
                } else {
                    EventResponse::NONE
                }
            }
            Event::MouseUp(_) => {
                if self.1 {
                    self.1 = false;
                    EventResponse::changed(self.2, true)
                } else {
                    EventResponse::NONE
                }
            }
            Event::MouseExit => {
                if self.1 {
                    self.1 = false;
                    EventResponse::changed(self.2, true)
                } else {
                    EventResponse::NONE
                }
            }
            _ => EventResponse::NONE,
        }
    }

    fn get_pref_size(&mut self, _context: WidgetContext, _data: &u32) -> PrefSize {
        PrefSize::min_max(Size::new(50.0, 50.0), Size::new(100.0, 100.0))
    }

    fn layout(&mut self, size: Size, _context: WidgetContext, _data: &u32) {
        self.2 = size;
    }

    fn build(&mut self, _context: WidgetContext) {}
}

fn test_layout(spacing: Spacing, index: u32) -> impl Widget<u32> {
     Container::new(HBox::new(spacing, 0.0))
        .child(ColorRect::new(Color::rgb8(180, 0, 0), index + 1))
        .child(
            ColorRect::new(Color::rgb8(60, 0, 150), index + 2)
                .loosen_size(PrefSize::flexible((150.0, 150.0)))
        )
        .child(ColorRect::new(Color::rgb8(0, 200, 200), index + 3))
        .child(ColorRect::new(Color::rgb8(0, 200, 0), index + 4))
     .background(Color::grey8(30), 0.0)
}

fn main() {
    WindowBuilder::new()
        .open(
            Container::new(VBox::new(Spacing::Between, 5.0))
                .child(Label::new("Spacing::Around"))
                .child(test_layout(Spacing::Around, 10))
                .child(Label::new("Spacing::Equal"))
                .child(test_layout(Spacing::Equal, 20))
                .child(Label::new("Spacing::Padding"))
                .child(test_layout(Spacing::Padding, 30))
                .child(Label::new("Spacing::Between"))
                .child(test_layout(Spacing::Between, 40))
            .background(Color::grey8(50), 10.0)
            .padding(20.0)
        )
}