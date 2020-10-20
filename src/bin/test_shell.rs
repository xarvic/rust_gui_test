use druid_shell::{Application, WindowBuilder, WinHandler, WindowHandle};
use druid_shell::kurbo::{Size, Circle, Rect};
use std::any::Any;
use druid_shell::piet::{Piet, RenderContext, Brush, Color};
use std::time::Instant;

struct Handler(Instant);

impl WinHandler for Handler {
    fn connect(&mut self, handle: &WindowHandle) {
        println!("started in {} sec!", self.0.elapsed().as_secs_f32());
        handle.show();
    }

    fn paint(&mut self, piet: &mut Piet, _invalid_rect: Rect) -> bool {
        let brush = piet.solid_brush(Color::rgb8(255, 255, 0));
        piet.fill(Circle::new((300.0, 300.0), 100.0), &brush);
        false
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

fn main() {
    let app = Application::new().unwrap();
    let mut window = WindowBuilder::new(app.clone());

    let now = Instant::now();

    window.set_size(Size::new(500.0, 500.0));
    window.resizable(true);
    window.set_handler(Box::new(Handler(now)));
    window.build().unwrap();

    app.run(None);

}