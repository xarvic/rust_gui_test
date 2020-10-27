use gui::prelude::*;

fn main() {
    let text = CloneState::new(String::from("Test"));

    WindowBuilder::new()
        .title("Test Labels")
        .keep_min_size()
        .open(state(&text,
            Container::new(VBox::new(Spacing::Right, 10.0))
                .child(
                    DynLabel::new(|data: &String|data.clone())
                )
                .child(Spacer::fixed(0.0, 10.0))
                .child(
                    TextInput::new()
                    .background(Color::grey8(100), 2.0)
                )
        ))
}