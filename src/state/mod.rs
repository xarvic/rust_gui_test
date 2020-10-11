use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{RwLock, Arc};
use std::ops::Deref;

pub mod key;
pub mod lens;

mod manager;
pub use manager::sync_states;
pub(crate) use manager::update_state;
use std::any::Any;
use crate::state::manager::MANAGER;


#[derive(Copy, Clone, Hash, Ord, PartialOrd, PartialEq, Eq)]
pub struct StateID(u64);

pub(crate) struct StateInner<T> {
    id: StateID,
    commit: AtomicU64,
    value: RwLock<T>,
}

fn next() -> StateID {
    static ID: AtomicU64 = AtomicU64::new(0);
    let value = ID.load(Ordering::SeqCst);
    ID.compare_and_swap(value, value + 1, Ordering::SeqCst);
    StateID(value)
}

impl<T: Clone> StateInner<T> {
    fn new(value: T) -> Self {
        let id = next();
        StateInner{
            id,
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
    fn fetch_value(&self, value: &mut T) {
        value.clone_from(self.value.read().unwrap().deref());
    }
}

#[derive(Clone)]
pub struct State<T: Clone> {
    cache: T,
    commit: u64,
    inner: Arc<StateInner<T>>,
}

impl<T: 'static + Clone + Send + Sync> State<T> {
    pub fn new(value: T) -> Self {
        let state = State{
            cache: value.clone(),
            commit: 0,
            inner: Arc::new(StateInner::new(value)),
        };
        MANAGER.lock().unwrap().states.insert(state.id(), state.handle());
        state
    }

    pub fn fetch(&mut self) -> &T {
        let new_commit = self.inner.commit();
        if self.commit != new_commit {
            self.commit = new_commit;
            self.inner.fetch_value(&mut self.cache);
        }
        &self.cache
    }
    pub fn id(&self) -> StateID {
        self.inner.id()
    }

    pub fn handle(&self) -> Handle {
        Handle(self.inner.clone() as Arc<dyn HandleInner + Send + Sync>)
    }
}

pub(crate) trait HandleInner {
    fn update(&self, updater: Box<dyn FnOnce(&mut dyn Any)>);
    fn id(&self) -> StateID;
}

impl<T: 'static + Clone> HandleInner for StateInner<T> {
    fn update(&self, updater: Box<dyn FnOnce(&mut dyn Any)>) {
        updater(&mut*self.value.write().unwrap());
        self.commit.store(self.commit.load(Ordering::SeqCst) + 1, Ordering::SeqCst);
    }

    fn id(&self) -> StateID {
        self.id
    }
}

pub struct Handle(pub(crate) Arc<dyn HandleInner + Send + Sync>);