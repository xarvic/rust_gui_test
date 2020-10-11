use druid_shell::{Application, WindowBuilder, WinHandler, WindowHandle};
use druid_shell::kurbo::{Size, Circle, Rect};
use std::any::Any;
use druid_shell::piet::{Piet, RenderContext, Brush};

struct Handler;

impl WinHandler for Handler {
    fn connect(&mut self, handle: &WindowHandle) {
        println!("start!");
        handle.show();
    }

    fn paint(&mut self, piet: &mut Piet, _invalid_rect: Rect) -> bool {
        piet.fill(Circle::new((300.0, 300.0), 100.0), &Brush::Solid(1000));
        false
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

fn main() {
    let app = Application::new().unwrap();
    let mut window = WindowBuilder::new(app.clone());

    window.set_size(Size::new(500.0, 500.0));
    window.resizable(true);
    window.set_handler(Box::new(Handler));
    window.build().unwrap();

    app.run(None);

}