use crate::state::{StateID, State, update_state};
use crate::state::lens::{Lens, Empty, Product};

#[derive(Clone)]
pub struct Key<'a, T: Clone> {
    value: &'a T,
    state_id: StateID,
}

impl<'a, T: Clone + Send + Sync + 'static> Key<'a, T> {
    pub fn new(state: &'a mut State<T>) -> Self {
        let id = state.id();
        Key {
            value: state.fetch(),
            state_id: id
        }
    }
}
