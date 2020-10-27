use crate::state::{Handle, HandleInner, StateInner, StateID, State};
use std::sync::Arc;
use crate::state::key::Key;

#[derive(Clone)]
pub struct CloneState<T: Clone> {
    cache: T,
    commit: u64,
    inner: Arc<StateInner<T>>,
}

impl<T: 'static + Clone + Send + Sync> CloneState<T> {
    pub fn new(value: T) -> Self {
        let state = CloneState {
            cache: value.clone(),
            commit: 0,
            inner: Arc::new(StateInner::new(value)),
        };
        state
    }

    pub fn handle(&self) -> Handle {
        Handle(self.inner.clone() as Arc<dyn HandleInner + Send + Sync>)
    }
}

impl<T: 'static + Clone + Send + Sync> State<T> for CloneState<T> {
    fn get_id(&self) -> StateID {
        self.inner.id()
    }

    fn with_value<R>(&self, operation: impl FnOnce(&T) -> R) -> R {
        operation(&self.cache)
    }

    fn with_fetched_value<R>(&mut self, operation: impl FnOnce(&T, Option<&T>) -> R) -> R {
        let new_commit = self.inner.commit();
        if new_commit > self.commit {
            self.commit = new_commit;

            let CloneState{inner, cache, commit} = self;
            inner.use_value(|value|cache.clone_from(value))
        }
        operation(&self.cache, None)//TODO: change
    }

    fn with_key<R>(&mut self, operation: impl FnOnce(Key<T>) -> R) -> R {
        let new_commit = self.inner.commit();
        if new_commit > self.commit {
            self.commit = new_commit;
            let CloneState{inner, cache, commit} = self;
            inner.use_value(|value|cache.clone_from(value))
        }
        let mut change = false;
        let r = operation(Key::new(&mut self.cache, &mut change));

        if change {
            let (_, commit) = self.inner.update_value(|value| {
                value.clone_from(&self.cache)
            });
            self.commit = commit;
        }

        r
    }
}