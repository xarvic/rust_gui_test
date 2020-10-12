use pool_tree::tree::Tree;
use std::collections::HashMap;
use crate::state::StateID;
use druid_shell::kurbo::{Point, Size, Rect};
use crate::event::Event;
use druid_shell::piet::Piet;
use crate::widgets::state::StateWidget;

pub struct StateRoot {
    widgets: Box<dyn StateWidget>,
    position: Point,
    size: Size,
    last_update: u64,

}

impl StateRoot {

}

pub struct WidgetGraph {
    tree: Tree<StateRoot>,
    dependent_nodes: HashMap<StateID, Vec<u32>>,
    dirty_rect: Option<Rect>,
}

impl WidgetGraph {
    pub fn update(&mut self, states: &[StateID]) {

    }
    pub fn handle_event(&mut self, event: Event) {

    }
    pub fn dirty(&self) -> Option<Rect> {
        self.dirty_rect.clone()
    }
    pub fn layout(&mut self) {

    }
    pub fn draw_widgets(&mut self, piet: &mut Piet, dirty_rect: Rect) -> bool {
        false
    }
}