use gui::window::WindowBuilder;
use gui::widgets::layout::{Container, HBox, Spacing};
use gui::widgets::*;
use gui::state::CloneState;
use gui::widgets::text::{Label, DynLabel};
use gui::state::key::Key;

fn main() {

    let state1 = CloneState::new(20);
    let state2 = CloneState::new(0);

    WindowBuilder::new()
        .keep_min_size()
        .title("test states")
        .open(Container::new(HBox::new(Spacing::Around, 10.0))
            .child(Label::new("button:", None))
            .child(state(&state1,
                         button(DynLabel::new(|value: &u32|value.to_string()), None,
                                |_, mut state: Key<u32>|state.change(|value|*value -= 1)
                         )
            ))
            //.child(Spacer::grow())
            .child(Label::new("right:", None))
            .child(state(&state1,
                button(DynLabel::new(|value: &u32|value.to_string()), None,
                          |_, mut state: Key<u32>|state.change(|value|*value += 1)
                )
            ))
        )
}