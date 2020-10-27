use druid_shell::Counter;
use crate::state::key::Key;

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
    fn with_fetched_value<R>(&mut self, operation: impl FnOnce(&T, Option<&T>) -> R) -> R;

    /// returns a Key of the fetcht value of the state
    /// Cached State will try to update their Value
    fn with_key<R>(&mut self, operation: impl FnOnce(Key<T>) -> R) -> R;
}

#[derive(Copy, Clone, Hash, Ord, PartialOrd, PartialEq, Eq)]
pub struct StateID(u64);

impl StateID{
    pub fn new() -> Self {
        static IDS: Counter = Counter::new();
        StateID(IDS.next())
    }
}