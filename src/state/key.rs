use std::ops::Deref;
use crate::state::lens::Lens;

/// Key allows to change a value and informs the owner about it
/// The current Implementation isn't the final form but it is designed in a way, that the Interface
/// dont have to change much in the Future
pub struct Key<'a, T: Clone> {
    pub(crate) value: &'a mut T,
    pub(crate) changed_flag: &'a mut bool,
}

impl<'a, T: Clone> Key<'a, T> {

}

impl<'a, T: Clone> Key<'a, T> {
    pub fn new(value: &'a mut T, changed_flag: &'a mut bool) -> Self {
        Key{
            value,
            changed_flag,
        }
    }

    /// borrows the Key
    pub fn id(&mut self) -> Key<T> {
        Key {
            value: self.value,
            changed_flag: self.changed_flag,
        }
    }
    /// Changes the value of the associated State, the change will be visible after the next
    /// State-update
    // Note: this design is the most abstract, therefore we
    pub fn change<R>(&mut self, f: impl FnOnce(&mut T) -> R) -> R {
        *self.changed_flag = true;
        f(self.value)
    }

}

impl<'a, T: Clone + 'static> Key<'a, T> {
    /// Applies a Lens to the State
    /// this is done here, since we dont know if in the future Key will contain a mutable or
    /// immutable reference to Key
    pub fn with_lens<U: Clone + 'static, L: Lens<U, Source=T>, R>(&mut self, lens: L, operation: impl FnOnce(Key<U>) -> R) -> R {
        let Key{value, changed_flag} = self;
        lens.with_mut(value, |new_value|{
            operation(Key::new(new_value, changed_flag))
        })
    }
}

impl<'a, T: Clone> Deref for Key<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        &*self.value
    }
}