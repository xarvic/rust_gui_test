use std::collections::HashMap;
use crate::state::{StateID, CloneState, State};
use druid_shell::kurbo::{Point, Size, Rect};
use crate::event::{Event, Change};
use druid_shell::piet::Piet;
use crate::widgets::{Widget, StateRootWidget};
use crate::size::PrefSize;

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
    data: CloneState<u32>,
    tree: Box<dyn Widget<u32>>,
    //tree: Tree<StateRoot>,
    dependent_nodes: HashMap<StateID, Vec<u32>>,
    dirty_rect: Option<Rect>,
    size: Size,
    pref_size: PrefSize,
    re_layout: bool,
}

impl WidgetGraph {
    pub fn new<W: Widget<u32> + 'static>(root: W) -> Self {
        WidgetGraph {
            data: CloneState::new(0),
            tree: Box::new(root),
            dependent_nodes: HashMap::new(),
            dirty_rect: None,
            size: Size::ZERO,
            pref_size: PrefSize::zero(),
            re_layout: true,
        }
    }

    pub fn update(&mut self, states: &[StateID]) {

    }
    pub fn handle_event(&mut self, event: Event) {
        let tree = &mut self.tree;

        let response = self.data.with_key(|key|tree.handle_event(event, WidgetContext{}, key));

        match response.change() {
            Change::None => {}
            Change::Content(rect) => {
                self.dirty_rect = Some(self.dirty_rect.map_or(rect, |old|old.union(rect)));
                self.re_layout = true;
            }
            Change::Bounds => {
                self.dirty_rect = Some(Rect::new(0.0, 0.0, self.size.width, self.size.height));
            }
        }

    }
    pub fn dirty(&self) -> Option<Rect> {
        self.dirty_rect.clone()
    }
    pub fn pref_size(&mut self) -> PrefSize {
        self.pref_size
    }

    pub fn layout(&mut self, size: Size) {
        if self.size != size || self.re_layout {
            self.size = size;
            self.re_layout = false;

            let tree = &mut self.tree;


            self.pref_size = self.data.with_value(|value|{
                let pref = tree.get_pref_size(WidgetContext {}, value);
                tree.layout(size, WidgetContext {}, value);

                pref
            });
        }
    }
    pub fn draw_widgets(&mut self, piet: &mut Piet, size: Size, dirty_rect: Rect) -> bool {
        if self.dirty_rect
            .map_or(true, |rect|
                dirty_rect.contains(rect.origin()) &&
                    dirty_rect.contains(Point::new(rect.x1, rect.y1))) {
            self.dirty_rect = None;
        }

        let tree = &mut self.tree;

        self.data.with_value(|value| {
            tree.draw(piet, size, dirty_rect, WidgetContext {}, value);
        });
        false
    }
}