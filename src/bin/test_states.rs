use gui::window::WindowBuilder;
use gui::widgets::layout::{Container, HBox, Spacing};
use gui::widgets::*;
use gui::state::CloneState;
use gui::widgets::text::{Label, DynLabel};
use gui::state::key::Key;

fn main() {
    WindowBuilder::new()
        .keep_min_size()
        .title("test states")
        .open(state(
            CloneState::new(0u32),
            Container::new(HBox::new(Spacing::Around, 10.0))
            .child(Label::new("button:", None))
            .child(button(DynLabel::new(|value: &u32|value.to_string()), None,
                          |_, mut state: Key<u32>|state.change(|value|*value += 1)
            ))
            .child(Label::new("right:", None))
                .child(button(DynLabel::new(|value: &u32|value.to_string()), None,
                              |_, mut state: Key<u32>|state.change(|value|*value += 1)
                ))
        ))
}