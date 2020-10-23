use crate::widgets::style::Style;
use std::sync::Arc;
use crate::widgets::raw::{Wrapper, WrapperResponse};
use druid_shell::kurbo::{Rect, Size};
use crate::event::Event;
use druid_shell::piet::Piet;
use ccl_owning_ref::OwningRef;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Sheet{
    Idle,
    Focused,
    Hovered,
    HoveredFocused,
    Pressed,
    Active,
}

#[derive(Clone)]
pub struct StyleAtlas {
    pub idle: Style,
    pub focused: Option<Style>,
    pub hovered: Option<Style>,
    pub hovered_focused: Option<Style>,
    pub pressed: Option<Style>,
    pub active: Option<Style>,

    max_padding: f64,
}

impl StyleAtlas {
    pub fn new(idle: Style) -> Self {
        let padding = idle.full_padding();
        StyleAtlas {
            idle,
            focused: None,
            hovered: None,
            hovered_focused: None,
            pressed: None,
            active: None,
            max_padding: padding,
        }
    }

    fn sheet(&self, state: State) -> Sheet {
        if state.mouse_state == MouseState::Clicked && self.pressed.is_some() {
            return Sheet::Pressed;
        }
        if state.mouse_state != MouseState::Idle {
            if state.focused {
                if self.hovered_focused.is_some() {
                    return Sheet::HoveredFocused;
                } else if self.hovered.is_some() {
                    return Sheet::Hovered;
                }
            } else if self.hovered.is_some() {
                return Sheet::Hovered;
            }
        } else if state.active {
            if self.active.is_some() {
                return Sheet::Active;
            }
        } else if state.focused {
            if self.focused.is_some() {
                return Sheet::Focused;
            }
        }
        Sheet::Idle
    }
    fn get(self: Arc<Self>, sheet: Sheet) -> OwningRef<Arc<Self>, Style> {
        OwningRef::new(self).map(|this|{
            match sheet {
                Sheet::Idle => {
                    &this.idle
                }
                Sheet::Focused => {
                    this.focused.as_ref().unwrap_or(&this.idle)
                }
                Sheet::Hovered => {
                    this.hovered.as_ref().unwrap_or(&this.idle)
                }
                Sheet::HoveredFocused => {
                    this.hovered_focused.as_ref().unwrap_or(&this.idle)
                }
                Sheet::Pressed => {
                    this.pressed.as_ref().unwrap_or(&this.idle)
                }
                Sheet::Active => {
                    this.active.as_ref().unwrap_or(&this.idle)
                }
            }
        })
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum MouseState {
    Idle,
    Hovered,
    Clicked,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct State{
    focused: bool,
    active: bool,
    mouse_state: MouseState,
}

impl State {
    pub fn new() -> Self {
        State{
            focused: false,
            active: false,
            mouse_state: MouseState::Idle,
        }
    }
}

#[derive(Clone)]
pub struct DynamicStyle {
    atlas: OwningRef<Arc<StyleAtlas>, Style>,
    stable_padding: bool,
    state: State,
}

impl DynamicStyle {
    pub fn new(atlas: Arc<StyleAtlas>, stable_padding: bool) -> Self {
        let atlas = OwningRef::new(atlas).map(|atlas|&atlas.idle);

        DynamicStyle {
            atlas,
            stable_padding,
            state: State::new()
        }
    }
}

impl Wrapper for DynamicStyle {
    fn draw(&self, piet: &mut Piet, size: Size, dirty_rect: Rect) {
        self.atlas.draw(piet, size)
    }

    fn handle_event(&mut self, event: Event) -> WrapperResponse {
        let mut new_state = self.state;
        match event {
            Event::MouseEnter(_) => {
                if new_state.mouse_state == MouseState::Idle {
                    new_state.mouse_state = MouseState::Hovered;
                }
            }
            Event::MouseDown(_) => {
                new_state.mouse_state = MouseState::Clicked;
            }
            Event::MouseUp(_) => {
                if new_state.mouse_state == MouseState::Clicked {
                    new_state.mouse_state = MouseState::Hovered;
                }
            }
            Event::MouseExit => {
                new_state.mouse_state = MouseState::Idle;
            }
            _ => {}
        }
        if new_state != self.state {
            let new_sheet = self.atlas.as_owner().sheet(new_state);
            if new_sheet != self.atlas.as_owner().sheet(self.state) {
                self.state = new_state;
                update(&mut self.atlas ,|this|this.into_owner().get(new_sheet));
                return WrapperResponse::Changed;
            }
        }
        WrapperResponse::Unchanged
    }

    fn get_left_top_padding(&self) -> Size {
        let padding = if self.stable_padding {
            self.atlas.as_owner().max_padding
        } else {
            self.atlas.full_padding()
        };
        Size::new(padding, padding)
    }

    fn get_right_bottom_padding(&self) -> Size {
        self.get_left_top_padding()
    }
}

fn update<T, F: FnOnce(T) -> T>(value: &mut T, updater: F) {
    unsafe {
        std::ptr::write(value, updater(std::ptr::read(value)));
    }
}