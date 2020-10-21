use std::collections::HashMap;
use crate::state::{StateID, State};
use druid_shell::kurbo::{Point, Size, Rect};
use crate::event::Event;
use druid_shell::piet::Piet;
use crate::widgets::state::StateRootWidget;
use crate::widgets::{Widget, PrefSize};
use crate::state::key::Key;

pub struct StateRoot {
    widgets: Box<dyn StateRootWidget>,
    position: Point,
    size: Size,
    last_update: u64,

}

impl StateRoot {

}

pub struct WidgetContext {

}

impl WidgetContext {
    pub fn id(&self) -> Self {
        WidgetContext{}
    }
}

pub struct WidgetGraph {
    data: State<u32>,
    tree: Box<dyn Widget<u32>>,
    //tree: Tree<StateRoot>,
    dependent_nodes: HashMap<StateID, Vec<u32>>,
    dirty_rect: Option<Rect>,
}

impl WidgetGraph {
    pub fn new<W: Widget<u32> + 'static>(root: W) -> Self {
        WidgetGraph {
            data: State::new(0),
            tree: Box::new(root),
            dependent_nodes: HashMap::new(),
            dirty_rect: None,
        }
    }

    pub fn update(&mut self, states: &[StateID]) {

    }
    pub fn handle_event(&mut self, event: Event) {
        let key = Key::new(&mut self.data);
        self.tree.handle_event(event, WidgetContext{}, key);

    }
    pub fn dirty(&self) -> Option<Rect> {
        self.dirty_rect.clone()
    }
    pub fn pref_size(&mut self) -> PrefSize {
        self.tree.get_pref_size(WidgetContext{}, self.data.read())
    }

    pub fn layout(&mut self, size: Size) {
        self.tree.get_pref_size(WidgetContext{}, self.data.read());
        self.tree.layout(size, WidgetContext{}, self.data.read())
    }
    pub fn draw_widgets(&mut self, piet: &mut Piet, size: Size, dirty_rect: Rect) -> bool {
        self.tree.draw(piet, size, dirty_rect, WidgetContext{}, self.data.read());
        false
    }
}