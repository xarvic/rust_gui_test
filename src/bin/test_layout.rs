use gui::window::WindowBuilder;
use gui::widgets::{Widget, PrefSize, Label};
use druid_shell::piet::{Color, Piet, RenderContext};
use druid_shell::kurbo::{Rect, Size};
use gui::widget_graph::WidgetContext;
use gui::event::Event;
use gui::state::key::Key;
use gui::widgets::layout::{HBox, Container, Spacing, VBox, padding};

struct ColorRect(Color, bool);

impl Widget<u32> for ColorRect {
    fn draw(&mut self, painter: &mut Piet, size: Size, _dirty_rect: Rect, _context: WidgetContext, _data: &u32) {
        let brush = painter.solid_brush(if self.1 {Color::rgb8(100, 100, 255)} else {self.0.clone()});
        painter.stroke(Rect::new(5.0, 5.0, size.width - 5.0, size.height - 5.0), &brush, 10.0);
    }

    fn handle_event(&mut self, event: Event, _context: WidgetContext, _data: Key<u32>) {
        match event {
            Event::MouseEnter(_) => {
                println!("enter!");
            }
            Event::MouseDown(_) => {
                self.1 = true;
            }
            Event::MouseUp(_) => {
                self.1 = false;
                println!("clicked!");
            }
            Event::MouseExit => {
                self.1 = false;
                println!("exit!");
            }
            _ => {}
        }
    }

    fn get_pref_size(&mut self, _context: WidgetContext, _data: &u32) -> PrefSize {
        PrefSize::min_max(Size::new(100.0, 50.0), Size::new(150.0, 150.0))
    }

    fn layout(&mut self, _size: Size, _context: WidgetContext, _data: &u32) {}

    fn build(&mut self, _context: WidgetContext) {}
}

fn test_layout(spacing: Spacing) -> impl Widget<u32> {
    Container::new(HBox::new(spacing, 0.0))
        .child(ColorRect(Color::rgb8(244, 80, 0), false))
        .child(ColorRect(Color::rgb8(244, 244, 0), false))
        .child(ColorRect(Color::rgb8(0, 244, 100), false))
        .child(ColorRect(Color::rgb8(100, 0, 240), false))
}

fn main() {
    WindowBuilder::new()
        .open(
            padding(20.0, Container::new(VBox::new(Spacing::Between, 10.0))
                .child(Label::new("Spacing::Around"))
                .child(test_layout(Spacing::Around))
                .child(Label::new("Spacing::Equal"))
                .child(test_layout(Spacing::Equal))
                .child(Label::new("Spacing::Padding"))
                .child(test_layout(Spacing::Padding))
                .child(Label::new("Spacing::Between"))
                .child(test_layout(Spacing::Between))

        ))
}