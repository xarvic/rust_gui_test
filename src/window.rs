use druid_shell::{WinHandler, WindowHandle, MouseEvent, Application};
use druid_shell::piet::Piet;
use std::any::Any;
use druid_shell::kurbo::{Size, Rect};
use crate::widget_graph::WidgetGraph;
use crate::state::{sync_states, StateID};
use crate::event::Event;
use std::sync::mpsc::{Receiver, sync_channel};
use crate::widgets::Widget;
use crate::size::PrefSize;

struct Window{
    widgets: WidgetGraph,
    window_handle: Option<WindowHandle>,
    mouse_focus: bool,
    min_size: MinSize,
    title: String,
    change_queue: Receiver<StateID>,
    size: Size,
}

impl Window {
    pub fn new(size: Size, widget: impl Widget<u32> + 'static, change_queue: Receiver<StateID>, min_size: MinSize, title: String) -> Self {
        Window {
            widgets: WidgetGraph::new(widget),
            window_handle: None,
            mouse_focus: false,
            min_size,
            title,
            change_queue,
            size
        }
    }

    fn update_states(&mut self) {
        sync_states();
        let changes: Vec<_> = self.change_queue.try_iter().collect();
        self.widgets.update(&changes);
        self.update_widgets();
    }

    fn update_widgets(&mut self) {
        self.widgets.layout(self.size);

        if let Some((rect, handle)) = self.widgets.dirty().zip(self.window_handle.as_ref()) {
            handle.invalidate_rect(rect);
        }
    }

    fn handle_event(&mut self, event: Event) {
        self.widgets.handle_event(event);

        self.update_states();
    }
    fn widgets_pref_size(&mut self) -> PrefSize {
        self.widgets.pref_size()
    }
}

impl WinHandler for Window {
    fn connect(&mut self, handle: &WindowHandle) {
        self.window_handle = Some(handle.clone());
        handle.show();
        println!("Opened Window '{}'!", self.title);
        self.update_states();
    }

    fn size(&mut self, size: Size) {
        let old_size = self.size;
        self.size = size;

        self.widgets.layout(size);
    }

    fn paint(&mut self, piet: &mut Piet, invalid_rect: Rect) -> bool {
        self.widgets.draw_widgets(piet, self.size, invalid_rect)
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

#[derive(Copy, Clone)]
pub enum MinSize {
    Fixed(Size),
    WidgetsMinSize,
}

pub struct WindowBuilder {
    size: Size,
    title: String,
    min_size: MinSize,
}

impl WindowBuilder {
    pub fn new() -> Self {
        WindowBuilder {
            size: Size::new(640.0, 480.0),
            title: "App".to_string(),
            min_size: MinSize::Fixed(Size::ZERO),
        }
    }
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }
    pub fn keep_min_size(mut self) -> Self {
        self.min_size = MinSize::WidgetsMinSize;
        self
    }
    pub fn keep_fixed_min_size(mut self, size: Size) -> Self {
        self.min_size = MinSize::Fixed(size);
        self
    }

    pub fn open<W: Widget<u32> + 'static>(self, widget: W) {
        println!("create window '{}'", &self.title);

        //Configure Widgets

        let app = Application::new().unwrap();

        let (sender, reciever) = sync_channel(100);

        let mut handler = Window::new(self.size,
                                      widget,
                                      reciever,
                                      self.min_size.clone(),
                                      self.title.clone());

        let pref_size = handler.widgets_pref_size();

        let min_size = match &self.min_size {
            MinSize::Fixed(size) => size.clone(),
            MinSize::WidgetsMinSize => Size::new(pref_size.min.width + 20.0,
                                                 pref_size.min.height + 20.0),
            //TODO: change this to pref_size.min when we can change the min size of an existing Window
            //This works since most Windows dont change their pref_size much
        };

        println!("min size: {}!", pref_size.min);

        //Create Platform Window

        let mut window = druid_shell::WindowBuilder::new(app.clone());
        window.set_size(Size::new(pref_size.max.width.max(200.0), pref_size.max.width.max(150.0)));
        window.resizable(true);
        window.set_title(self.title);
        window.set_min_size(Size::new(min_size.width.max(180.0), min_size.width.max(50.0)));
        window.set_handler(Box::new(handler));
        window.build().unwrap();

        app.run(None)
    }
}