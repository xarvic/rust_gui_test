use druid_shell::{WinHandler, WindowHandle, MouseEvent, Application, IdleToken, KeyEvent, KeyCode};
use druid_shell::piet::Piet;
use std::any::Any;
use druid_shell::kurbo::{Size, Rect};
use crate::widget_graph::WidgetGraph;
use crate::state::{StateID, register_listener};
use crate::event::Event;
use std::sync::mpsc::Receiver;
use crate::widgets::Widget;
use crate::size::PrefSize;

struct Window{
    widgets: WidgetGraph,
    window_handle: Option<WindowHandle>,
    mouse_focus: bool,
    min_size: MinSize,
    title: String,
    change_queue: Option<Receiver<StateID>>,
    size: Size,
}

impl Window {
    pub fn new(size: Size, widget: impl Widget<()> + 'static, min_size: MinSize, title: String) -> Self {
        Window {
            widgets: WidgetGraph::new(widget),
            window_handle: None,
            mouse_focus: false,
            min_size,
            title,
            change_queue: None,
            size
        }
    }

    fn update_states(&mut self) {
        if let Some(changes) = self.change_queue.as_ref() {
            let changes= changes.try_iter().collect::<Vec<_>>();
            self.widgets.update(&changes);
        }
        self.update_widgets();
    }

    fn update_widgets(&mut self) {
        self.widgets.layout(self.size);

        if let Some((rect, handle)) = self.widgets.dirty().zip(self.window_handle.as_ref()) {
            handle.invalidate_rect(rect);
        }
    }

    fn handle_event(&mut self, event: Event) -> bool {
        let response = self.widgets.handle_event(event);

        self.update_states();
        response.is_consumed()
    }
    fn widgets_pref_size(&mut self) -> PrefSize {
        self.widgets.pref_size()
    }
}

impl WinHandler for Window {
    fn connect(&mut self, handle: &WindowHandle) {
        self.window_handle = Some(handle.clone());
        handle.show();

        {
            let handle = handle.clone();

            let (listener, id) = register_listener(Some(Box::new(move|| {
                handle.get_idle_handle().unwrap().schedule_idle(IdleToken::new(0))
            })));
            self.change_queue = Some(listener);

        }

        println!("opened window '{}'", self.title);
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

    fn key_down(&mut self, event: KeyEvent) -> bool {
        //TODO: change, this is only temporary
        if event.key_code == KeyCode::Tab {
            self.widgets.traverse_focus();
            self.update_states();
            return true;
        }
        self.handle_event(Event::KeyDown(event.clone()))
    }

    fn key_up(&mut self, event: KeyEvent) {
        self.handle_event(Event::KeyUp(event.clone()));
    }

    fn idle(&mut self, token: IdleToken) {
        self.update_states()
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

    pub fn open<W: Widget<()> + 'static>(self, widget: W) {
        println!("create window '{}'", &self.title);

        //Configure Widgets

        let app = Application::new().unwrap();

        let mut handler = Window::new(self.size,
                                      widget,
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

        //Create Platform Window

        let mut window = druid_shell::WindowBuilder::new(app.clone());
        window.set_size(Size::new(pref_size.max.width.max(200.0), pref_size.max.width.max(150.0)));
        window.resizable(true);
        window.set_title(self.title);
        window.set_min_size(Size::new(min_size.width.max(150.0), min_size.height.max(50.0)));
        window.set_handler(Box::new(handler));
        window.build().unwrap();

        app.run(None)
    }
}