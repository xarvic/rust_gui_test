use druid_shell::{WinHandler, WindowHandle, MouseEvent, Application};
use druid_shell::piet::Piet;
use std::any::Any;
use druid_shell::kurbo::{Size, Rect};
use crate::widget_graph::WidgetGraph;
use crate::state::{sync_states, StateID};
use crate::event::Event;
use std::sync::mpsc::{Receiver, sync_channel};
use crate::widgets::Widget;

struct Window{
    widgets: WidgetGraph,
    window_handle: Option<WindowHandle>,
    mouse_focus: bool,
    scale_to_widgets_min_size: bool,
    change_queue: Receiver<StateID>,
    size: Size,
}

impl Window {
    pub fn new(size: Size, widget: impl Widget<u32> + 'static, change_queue: Receiver<StateID>) -> Self {
        Window {
            widgets: WidgetGraph::new(widget),
            window_handle: None,
            mouse_focus: false,
            scale_to_widgets_min_size: false,
            change_queue,
            size
        }
    }

    fn update_widgets(&mut self) {
        sync_states();
        let changes: Vec<_> = self.change_queue.try_iter().collect();
        self.widgets.update(&changes);
        self.widgets.layout(self.size);
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
        println!("Opened Window!");
        self.window_handle = Some(handle.clone());

        handle.show();

        self.update_widgets();
    }

    fn size(&mut self, size: Size) {
        let old_size = self.size;
        self.size = size;

        self.widgets.layout(size);
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

pub struct WindowBuilder {
    size: Size,
    title: String,
}

impl WindowBuilder {
    pub fn new() -> Self {
        WindowBuilder {
            size: Size::new(640.0, 480.0),
            title: "App".to_string()
        }
    }
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }
    pub fn open<W: Widget<u32> + 'static>(self, widget: W) {
        println!("open window '{}'", &self.title);
        let app = Application::new().unwrap();

        let (sender, reciever) = sync_channel(100);

        let handler = Window::new(self.size, widget, reciever);

        let mut window = druid_shell::WindowBuilder::new(app.clone());
        window.set_size(self.size);
        window.resizable(true);
        window.set_handler(Box::new(handler));
        window.build().unwrap();

        app.run(None)
    }
}