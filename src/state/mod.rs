use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{RwLock, Arc};
use std::ops::{Deref, DerefMut};

pub mod key;
pub mod lens;

mod manager;
mod clone_state;
mod sync_state;

pub use manager::sync_states;
pub use clone_state::CloneState;

use std::any::Any;
use druid_shell::Counter;
use crate::state::key::Key;


#[derive(Copy, Clone, Hash, Ord, PartialOrd, PartialEq, Eq)]
pub struct StateID(u64);

impl StateID{
    pub fn new() -> Self {
        static IDS: Counter = Counter::new();
        StateID(IDS.next())
    }
}

pub(crate) struct StateInner<T> {
    id: StateID,
    commit: AtomicU64,
    value: RwLock<T>,
}



impl<T: Clone> StateInner<T> {
    fn new(value: T) -> Self {
        StateInner{
            id: StateID::new(),
            commit: AtomicU64::new(0),
            value: RwLock::new(value),
        }
    }
    fn id(&self) -> StateID {
        self.id
    }

    fn commit(&self) -> u64 {
        self.commit.load(Ordering::SeqCst)
    }

    fn use_value<R>(&self, operation: impl FnOnce(&T) -> R) -> R {
        operation(self.value.read().unwrap().deref())
    }

    /// Executes operation with the value and increases the commit
    /// Returns the return value of the operation and the new commit!
    fn update_value<R>(&self, operation: impl FnOnce(&mut T) -> R) -> (R, u64) {
        let r = operation(self.value.write().unwrap().deref_mut());
        let old = self.commit.fetch_add(1, Ordering::SeqCst);
        (r, old + 1)
    }
    /// Executes operation with value. the commit value increases if operation returns true
    /// Returns the return value of the operation and the new commit!
    ///
    /// #Safety
    /// Other States wont notice the Statechange if operation changes the value, but doesnt return
    /// true.
    /// Key can solve this Problem!
    unsafe fn update_maybe<R>(&self, operation: impl FnOnce(&mut T) -> (R, bool)) -> (R, u64) {
        unimplemented!()
    }

}

/// This is the common Interface for all States to access their values
///
pub trait State<T> {
    /// Returns the Unique StateID of this State
    fn get_id(&self) -> StateID;

    /// Returns the value of the State
    /// Cached States wont try to update their Values
    fn with_value<R>(&self, operation: impl FnOnce(&T) -> R) -> R;

    /// returns the value of the State
    /// Cached State will try to update their Value
    fn with_fetched_value<R>(&mut self, operation: impl FnOnce(&T) -> R) -> R;

    /// returns a Key of the fetcht value of the state
    /// Cached State will try to update their Value
    fn with_key<R>(&mut self, operation: impl FnOnce(Key<T>) -> R) -> R;
}

pub(crate) trait HandleInner {
    fn update(&self, updater: Box<dyn FnOnce(&mut dyn Any)>);
    fn id(&self) -> StateID;
}

impl<T: 'static + Clone> HandleInner for StateInner<T> {
    fn update(&self, updater: Box<dyn FnOnce(&mut dyn Any)>) {
        self.update_value(|value|{
            updater(value as &mut dyn Any)
        });
    }

    fn id(&self) -> StateID {
        self.id
    }
}

pub struct Handle(pub(crate) Arc<dyn HandleInner + Send + Sync>);