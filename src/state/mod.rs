use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{RwLock, Arc};
use std::ops::Deref;

pub mod key;
pub mod lens;

mod manager;
pub use manager::sync_states;
pub(crate) use manager::update_state;
use std::any::Any;


#[derive(Copy, Clone)]
pub struct StateID(u64);

pub(crate) struct StateInner<T> {
    id: StateID,
    commit: AtomicU64,
    value: RwLock<T>,
}

impl<T: Clone> StateInner<T> {
    fn new(value: T) -> Self {
        StateInner{
            id: StateID(0),//TODO: implement global counter
            commit: AtomicU64::new(0),
            value: RwLock::new(value),
        }
    }
    fn id(&self) -> StateID {
        self.id
    }
    fn commit(&self) -> u64 {
        self.commit.load(Ordering::Relaxed)
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

impl<T: Clone> State<T> {
    pub fn new(value: T) -> Self {
        State{
            cache: value.clone(),
            commit: 0,
            inner: Arc::new(StateInner::new(value)),
        }
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
}

trait StateUpdate{
    fn update(&self, updater: Box<dyn Fn(&mut dyn Any)>);
    fn id(&self) -> StateID;
}

impl<T: Clone> StateUpdate for State<T> {
    fn update(&self, updater: Box<dyn Fn(&mut dyn Any)>) {
        unimplemented!()
    }

    fn id(&self) -> StateID {
        unimplemented!()
    }
}