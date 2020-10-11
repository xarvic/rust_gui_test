use std::any::Any;
use crate::state::{StateID, StateHandle};
use std::sync::mpsc::{Receiver, SyncSender, sync_channel};
use once_cell::sync::Lazy;
use std::thread::spawn;
use std::sync::atomic::{AtomicBool, Ordering};
use std::collections::{VecDeque, HashMap};
use std::sync::Mutex;

pub(crate) fn update_state<T: 'static, F: FnOnce(&mut T)>(id: StateID, update: F) {
    let _update = |value: &mut dyn Any|{
        if let Some(value) = value.downcast_mut::<T>() {
            update(value);
        } else {
            eprintln!("update closure for State({}) with a wrong type was provided!", {id.0});
        }
    };
}

/// applies all changes to the States which were created by Key::update
/// if multiple threads call this function all all wait until the States are finished
pub fn sync_states() {
    if CHANGED.load(Ordering::SeqCst) {
        let mut manager = MANAGER.lock().unwrap();

        while let Some((id, update)) = manager.updates.pop_front() {
            if let Some(state) = manager.states.get(&id) {
                update
            }
        }
    }
}

type StateUpdate = (StateID, Box<dyn Fn(&mut dyn Any) + Send>);

struct Manager{
    updates: VecDeque<StateUpdate>,
    states: HashMap<StateID, StateHandle>,
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

static MANAGER: Lazy<Mutex<Manager>> = Lazy::new(||Mutex::new(Manager::new()));