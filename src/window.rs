use druid_shell::{WinHandler, WindowHandle, MouseEvent};
use druid_shell::piet::Piet;
use std::any::Any;
use druid_shell::kurbo::{Size, Rect};
use crate::widget_graph::WidgetGraph;
use crate::state::{sync_states, StateID};
use crate::event::Event;
use std::sync::mpsc::Receiver;

struct Window{
    widgets: WidgetGraph,
    window_handle: Option<WindowHandle>,
    mouse_focus: bool,
    scale_to_widgets_min_size: bool,
    change_queue: Receiver<StateID>,
}

impl Window {
    fn update_widgets(&mut self) {
        sync_states();
        let changes: Vec<_> = self.change_queue.try_iter().collect();
        self.widgets.update(&changes);
        self.widgets.layout();
        if let Some((rect, handle)) = self.widgets.dirty().zip(self.window_handle.as_ref()) {
            handle.invalidate_rect(rect);
        }
    }

    fn handle_event(&mut self, event: Event) {
        self.widgets.handle_event(event);

        self.update_widgets();
    }
}

impl WinHandler for Window {
    fn connect(&mut self, handle: &WindowHandle) {
        self.window_handle = Some(handle.clone());

        self.update_widgets();
    }

    fn size(&mut self, size: Size) {
        unimplemented!()
    }

    fn paint(&mut self, piet: &mut Piet, invalid_rect: Rect) -> bool {
        println!("Repaint Window!");
        self.widgets.draw_widgets(piet, invalid_rect)
    }

    fn mouse_move(&mut self, event: &MouseEvent) {
        if !self.mouse_focus {
            self.handle_event(Event::MouseEnter(event.clone()));
            self.mouse_focus = true;
        }
        self.handle_event(Event::MouseMove(event.clone()));
    }

    fn mouse_down(&mut self, event: &MouseEvent) {
        if !self.mouse_focus {
            self.handle_event(Event::MouseEnter(event.clone()));
            self.mouse_focus = true;
        }
        self.handle_event(Event::MouseDown(event.clone()));
    }

    fn mouse_up(&mut self, event: &MouseEvent) {
        if !self.mouse_focus {
            self.handle_event(Event::MouseEnter(event.clone()));
            self.mouse_focus = true;
        }
        self.handle_event(Event::MouseUp(event.clone()));
    }

    fn mouse_leave(&mut self) {
        if self.mouse_focus {
            self.handle_event(Event::MouseExit);
            self.mouse_focus = false;
        }
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

struct WindowBuilder{

}