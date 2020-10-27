use crate::state::{StateID, update};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{RwLock, Arc};
use std::ops::{Deref, DerefMut};
use std::any::Any;

pub struct StateInner<T> {
    id: StateID,
    commit: AtomicU64,
    value: RwLock<T>,
}


impl<T: Clone> StateInner<T> {
    pub fn new(value: T) -> Self {
        StateInner{
            id: StateID::new(),
            commit: AtomicU64::new(0),
            value: RwLock::new(value),
        }
    }
    pub fn id(&self) -> StateID {
        self.id
    }

    pub fn commit(&self) -> u64 {
        self.commit.load(Ordering::SeqCst)
    }

    pub fn use_value<R>(&self, operation: impl FnOnce(&T) -> R) -> R {
        operation(self.value.read().unwrap().deref())
    }

    /// Executes operation with the value and increases the commit
    /// Returns the return value of the operation and the new commit!
    pub fn update_value<R>(&self, operation: impl FnOnce(&mut T) -> R) -> (R, u64) {
        let r = operation(self.value.write().unwrap().deref_mut());
        let old = self.commit.fetch_add(1, Ordering::SeqCst);
        update(self.id);
        (r, old + 1)
    }
    /// Executes operation with value. the commit value increases if operation returns true
    /// Returns the return value of the operation and the new commit!
    ///
    /// #Safety
    /// Other States wont notice the Statechange if operation changes the value, but doesnt return
    /// true.
    /// Key can solve this Problem!
    pub unsafe fn update_maybe<R>(&self, operation: impl FnOnce(&mut T) -> (R, bool)) -> (R, u64) {
        unimplemented!()
    }

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

pub trait HandleInner {
    fn update(&self, updater: Box<dyn FnOnce(&mut dyn Any)>);
    fn id(&self) -> StateID;
}

pub struct Handle(pub(crate) Arc<dyn HandleInner + Send + Sync>);