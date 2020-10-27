pub mod key;
pub mod lens;

mod manager;
mod clone_state;
mod sync_state;
mod state_inner;
mod state;

pub use manager::{register_listener, unregister_listener};
pub(crate) use manager::update;
pub use clone_state::CloneState;
pub use state_inner::{StateInner, HandleInner, Handle};
pub use state::{StateID, State};