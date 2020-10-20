use gui::window::WindowBuilder;
use gui::widgets::{Empty, Widget, PrefSize};
use druid_shell::piet::{Color, Piet, RenderContext};
use druid_shell::kurbo::{Rect, Size};
use gui::widget_graph::WidgetContext;
use gui::event::Event;
use gui::state::key::Key;
use gui::widgets::layout::{Container, HBox};

struct ColorRect(Color);

impl Widget<u32> for ColorRect {
    fn draw(&self, painter: &mut Piet, dirty_rect: Rect, context: WidgetContext, data: &u32) {
        let brush = painter.solid_brush(self.0.clone());
        painter.stroke(Rect::new(5.0, 5.0, 95.0, 95.0), &brush, 10.0);
    }

    fn handle_event(&mut self, event: Event, context: WidgetContext, data: Key<u32>) {
        if let Event::MouseUp(event) = event {
            println!("clicked!");
        }
    }

    fn get_pref_size(&self, context: WidgetContext, data: &u32) -> PrefSize {
        PrefSize::fixed(Size::new(100.0, 100.0))
    }

    fn layout(&mut self, size: Size, context: WidgetContext, data: &u32) {}

    fn build(&mut self, context: WidgetContext) {}
}

fn main() {
    ColorRect(Color::rgb8(244, 80, 0));
    WindowBuilder::new()
        .open(Container::new(HBox::new())
            .with(0, ColorRect(Color::rgb8(244, 80, 0)))
            .with(0, ColorRect(Color::rgb8(244, 244, 0)))
            .with(0, ColorRect(Color::rgb8(0, 244, 100)))
        )
}