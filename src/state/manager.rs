use crate::state::StateID;
use once_cell::sync::Lazy;
use druid_shell::Counter;
use std::sync::mpsc::{Receiver, SyncSender, sync_channel, Sender, channel};
use std::thread::spawn;

enum Action{
    StateUpdate(StateID),
    RegisterListener(Option<Box<dyn Fn() + Send>>, Sender<StateID>, ListenerID),
    UnregisterListener(ListenerID),
}

static LISTENER: Lazy<SyncSender<Action>> = Lazy::new(||{
    let (sender, reciever) = sync_channel::<Action>(100);

    spawn(move||{
        let mut data = Vec::<(Option<Box<dyn Fn() + Send>>, Sender<StateID>, ListenerID)>::new();

        for event in reciever.iter() {
            match event {
                Action::StateUpdate(state_id) => {
                    for (notify, states, _) in data.iter() {
                        states.send(state_id).unwrap();
                        println!("handle state update!");
                        if let Some(notify) = notify {
                            notify();
                        }
                    }
                }
                Action::RegisterListener(observer, listener, id) => {
                    data.push((observer, listener, id));
                }
                Action::UnregisterListener(listener) => {
                    data.retain(|(_, _, id)|id.0 != listener.0);
                }
            }
        }
    });

    sender
});

#[derive(Copy, Clone)]
pub struct ListenerID(u64);

fn next_id() -> ListenerID {
    static COUNTER: Counter = Counter::new();
    ListenerID(COUNTER.next())
}

pub fn register_listener(observer: Option<Box<dyn Fn() + Send>>) -> (Receiver<StateID>, ListenerID) {
    let (sender, reciever) = channel();
    let id = next_id();
    LISTENER.send(Action::RegisterListener(observer, sender, id)).unwrap();
    (reciever, id)
}

pub fn unregister_listener(id: ListenerID) {
    LISTENER.send(Action::UnregisterListener(id)).unwrap();
}

pub(crate) fn update(state: StateID) {
    LISTENER.send(Action::StateUpdate(state)).unwrap();
}