use gui::state::{State, sync_states};
use gui::state::key::Key;

fn main() {
    let mut state1 = State::new(0);

    let key = Key::new(&mut state1);

    key.update(|data|*data += 1);

    sync_states();

    println!("state: {}", *state1.fetch())

}