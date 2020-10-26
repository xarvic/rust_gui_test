use std::collections::HashMap;
use crate::state::{StateID, CloneState};
use druid_shell::kurbo::{Point, Size, Rect};
use crate::event::{Event, Change, EventResponse};
use druid_shell::piet::Piet;
use crate::widgets::{Widget, StateWidget, StateWidgetImpl};
use crate::size::PrefSize;
use pool_tree::children_unique::ChildrenUnique;
use pool_tree::tree::Tree;
use pool_tree::child_unique::ChildUniq;
use pool_tree::ref_unique::RefUniq;
use pool_tree::reference::Ref;
use pool_tree::reference::TreeRef;

pub struct StateRoot {
    widgets: Box<dyn StateWidget>,
    position: Point,
    size: Size,
    last_update: u64,
}

impl StateRoot {
    pub fn new(state: Box<dyn StateWidget>) -> Self {
        StateRoot {
            widgets: state,
            position: Point::ZERO,
            size: Size::ZERO,
            last_update: 0,
        }
    }

    pub fn draw(mut self: ChildUniq<Self>, piet: &mut Piet, size: Size, dirty_rect: Rect, env: Env) {
        let (this, childs) = self.get_both_unique();
        this.widgets.draw(piet, size, dirty_rect, WidgetContext::new(childs, env))
    }
    pub fn handle_event(mut self: ChildUniq<Self>, event: Event, env: Env) -> EventResponse {
        let (this, childs) = self.get_both_unique();
        this.widgets.handle_event(event, WidgetContext::new(childs, env))
    }
    pub fn get_pref_size(mut self: ChildUniq<Self>, env: Env) -> PrefSize {
        let (this, childs) = self.get_both_unique();
        this.widgets.get_pref_size(WidgetContext::new(childs, env))
    }
    pub fn layout(mut self: ChildUniq<Self>, size: Size, env: Env) {
        let (this, childs) = self.get_both_unique();
        this.widgets.layout(size, WidgetContext::new(childs, env))
    }
    pub fn traverse_focus(mut self: ChildUniq<Self>, env: Env) -> bool {
        let (this, childs) = self.get_both_unique();
        this.widgets.traverse_focus(WidgetContext::new(childs, env))
    }
    pub fn build(mut self: ChildUniq<Self>, env: Env) {
        let (this, childs) = self.get_both_unique();
        this.widgets.build(WidgetContext::new(childs, env))
    }
    pub fn states<'a>(self: Ref<'a, Self>) -> StateID {
        self.widgets.states()
    }
    pub fn update(self: RefUniq<Self>) {
        //TODO: implement
    }
}

pub struct Env<'a> {
    dependent_nodes: &'a mut HashMap<StateID, Vec<u32>>,
}

impl<'a> Env<'a> {
    pub fn new(dependent_nodes: &'a mut HashMap<StateID, Vec<u32>>) -> Self {
        Env {
            dependent_nodes
        }
    }
    pub fn id(&mut self) -> Env {
        Env {
            dependent_nodes: self.dependent_nodes,
        }
    }
}

pub struct WidgetContext<'a> {
    childs: ChildrenUnique<'a, StateRoot>,
    env: Env<'a>,
}

impl<'a> WidgetContext<'a> {
    pub fn new(childs: ChildrenUnique<'a, StateRoot>, env: Env<'a>) -> Self {
        WidgetContext{
            childs,
            env,
        }
    }
    pub fn id(&mut self) -> WidgetContext {
        WidgetContext{
            childs: self.childs.id(),
            env: self.env.id(),
        }
    }
    pub fn env(&mut self) -> Env {
        self.env.id()
    }
    pub(crate) fn insert<'b>(&'b mut self, root: StateRoot) -> (ChildUniq<'b, StateRoot>, Env<'b>, u32) {
        let index = self.childs.count();
        let child = self.childs.add_child(root);
        {
            let nodes = &mut self.env.dependent_nodes;
            let state = child.get_ref().states();
            nodes.entry(state).or_default().push(child.get_ref().index());
        }
        (child, self.env.id(), index)
    }
    pub(crate) fn get_child<'b>(&'b mut self, index: u32) -> (ChildUniq<'b, StateRoot>, Env<'b>) {
        (self.childs.get_child_unique(index), self.env.id())
    }
}

pub struct WidgetGraph {
    tree: Tree<StateRoot>,
    dependent_nodes: HashMap<StateID, Vec<u32>>,
    dirty_rect: Option<Rect>,
    size: Size,
    pref_size: PrefSize,
    re_layout: bool,
}

impl WidgetGraph {
    pub fn new<W: Widget<()> + 'static>(root: W) -> Self {

        let widget = StateWidgetImpl::new(CloneState::new(()), root);

        WidgetGraph {
            tree: Tree::new(StateRoot::new(Box::new(widget))),
            dependent_nodes: HashMap::new(),
            dirty_rect: None,
            size: Size::ZERO,
            pref_size: PrefSize::zero(),
            re_layout: true,
        }
    }

    pub fn update(&mut self, states: &[StateID]) {
        for state in states {

        }
    }
    pub fn handle_event(&mut self, event: Event) {

        let response = self.tree.mut_top().inner().handle_event(event, Env::new(&mut self.dependent_nodes));

        match response.change() {
            Change::None => {}
            Change::Content(rect) => {
                self.dirty_rect = Some(self.dirty_rect.map_or(rect, |old|old.union(rect)));
            }
            Change::Bounds => {
                self.re_layout = true;
            }
        }
    }
    pub fn dirty(&self) -> Option<Rect> {
        self.dirty_rect.clone()
    }

    pub fn re_layout(&self) -> bool {
        self.re_layout
    }

    pub fn pref_size(&mut self) -> PrefSize {
        self.tree.mut_top().inner().get_pref_size(Env::new(&mut self.dependent_nodes))
    }

    pub fn layout(&mut self, size: Size) {
        if self.size != size || self.re_layout {
            self.size = size;
            self.re_layout = false;

            self.pref_size = self.tree.mut_top().inner().get_pref_size(Env::new(&mut self.dependent_nodes));
            self.tree.mut_top().inner().layout(size, Env::new(&mut self.dependent_nodes));

            self.dirty_rect = Some(Rect::new(0.0, 0.0, self.size.width, self.size.height));
        }
    }
    pub fn draw_widgets(&mut self, piet: &mut Piet, size: Size, dirty_rect: Rect) -> bool {
        //Check if this draw command draw the whole dirty rect
        if self.dirty_rect
            .map_or(true, |rect|
                dirty_rect.contains(rect.origin()) &&
                    dirty_rect.contains(Point::new(rect.x1, rect.y1))) {
            self.dirty_rect = None;
        }

        self.tree.mut_top().inner().draw(piet, size, dirty_rect, Env::new(&mut self.dependent_nodes));
        false
    }
}