#![feature(const_fn)]
#![allow(unused_variables)]
#![allow(dead_code)]

pub use druid_shell;

pub mod widgets;
pub mod event;
pub mod state;
pub mod window;
pub mod widget_graph;
pub mod size;
pub mod text;

pub mod prelude{
    pub use crate::widgets::*;
    pub use crate::window::WindowBuilder::*;
    pub use crate::state::{Key, CloneState, State};
    pub use crate::event::*;
    pub use crate::size::PrefSize;

}