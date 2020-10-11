use crate::state::{StateID, State, update_state};
use crate::state::lens::{Lens, Empty, Product};

#[derive(Clone)]
pub struct Key<'a, F, T: Clone> where F: Lens<T>{
    lens: F,
    value: &'a T,
    state_id: StateID,
}

impl<'a, T: Clone + Send + Sync + 'static> Key<'a, Empty, T> {
    pub fn new(state: &'a mut State<T>) -> Self {
        let id = state.id();
        Key {
            lens: Empty,
            value: state.fetch(),
            state_id: id
        }
    }
}

impl<'a, F: Lens<T>, T: Clone + 'static> Key<'a, F, T> {
    pub fn update<G: FnOnce(&mut T) + Send + Sync + 'static>(&self, updater: G) {
        let f = self.lens.clone();
        update_state(self.state_id, move|value|f.lens_mut(value, updater));
    }

    pub fn with_lens<U: Clone, R, G: Lens<U, Source = T>>
    (&self, lens: G, operation: impl FnOnce(Key<Product<F, G>, U>) -> R) -> R {
        lens.lens(self.value, |value|{
            let key = Key {
                lens: Product(self.lens.clone(), lens.clone()),
                value,
                state_id: self.state_id
            };
            operation(key)
        })
    }
}

