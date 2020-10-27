#![feature(const_fn)]
#![allow(unused_variables)]
#![allow(dead_code)]

pub use druid_shell;

pub mod widgets;
pub mod state;
pub mod text;
pub mod app;

pub mod prelude{
    pub use crate::widgets::*;
    pub use crate::widgets::layout::{Container, VBox, HBox, Spacing};
    pub use crate::widgets::text::{Label, DynLabel, TextInput};
    pub use crate::state::{key::Key, CloneState, State};
    pub use crate::app::window::WindowBuilder;
    pub use crate::app::event::*;
    pub use crate::app::size::PrefSize;
    pub use ::druid_shell::piet::Color;
}