use gui::window::WindowBuilder;
use gui::widgets::layout::{Container, HBox, Spacing};
use gui::widgets::button;
use gui::widgets::text::Label;

fn main() {
    WindowBuilder::new()
        .keep_min_size()
        .title("test states")
        .open(Container::new(HBox::new(Spacing::Around, 0.0))
            .child(Label::new("button:", None))
            .child(button("Hi", None, |_, _|println!("hi")))
        )
}