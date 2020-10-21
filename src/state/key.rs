use crate::state::{StateID, State};

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

impl<'a, T: Clone> Key<'a, T> {
    pub fn id(&mut self) -> Key<T> {
        Key {
            value: self.value,
            state_id: self.state_id,
        }
    }
}