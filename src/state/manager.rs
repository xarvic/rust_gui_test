use std::any::Any;
use crate::state::{StateID, Handle};
use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicBool, Ordering};
use std::collections::{VecDeque, HashMap};
use std::sync::Mutex;

pub(crate) fn update_state<T: 'static, F: FnOnce(&mut T) + Send + Sync + 'static>(id: StateID, update: F) {
    let update = move|value: &mut dyn Any|{
        if let Some(value) = value.downcast_mut::<T>() {
            update(value);
        } else {
            eprintln!("update closure for State({}) with a wrong type was provided!", {id.0});
        }
    };
    let mut guard = MANAGER.lock().unwrap();

    CHANGED.store(true, Ordering::SeqCst);
    println!("push update!");
    guard.updates.push_back((id, Box::new(update)));
}

/// applies all changes to the States which were created by Key::update
/// if multiple threads call this function all all wait until the States are finished
pub fn sync_states() {
    if CHANGED.load(Ordering::SeqCst) {
        let mut manager = MANAGER.lock().unwrap();

        while let Some((id, update)) = manager.updates.pop_front() {
            println!("sync!");
            if let Some(state) = manager.states.get(&id) {
                state.0.update(update);
            } else {
                eprintln!("Updating nonexisting State {}!", id.0);
            }
        }
        CHANGED.store(false, Ordering::SeqCst);
    }
}

type StateUpdate = (StateID, Box<dyn FnOnce(&mut dyn Any) + Send>);

pub(crate) struct Manager{
    pub(crate) updates: VecDeque<StateUpdate>,
    pub(crate) states: HashMap<StateID, Handle>,
}

impl Manager{
    pub fn new() -> Self {
        Manager{
            updates: VecDeque::new(),
            states: HashMap::new(),
        }
    }
}

static CHANGED: AtomicBool = AtomicBool::new(false);

pub(crate) static MANAGER: Lazy<Mutex<Manager>> = Lazy::new(||Mutex::new(Manager::new()));