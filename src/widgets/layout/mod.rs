mod hbox;

pub use hbox::HBox;

use druid_shell::kurbo::{Size, Rect, Vec2, Affine};
use crate::widgets::{Widget, PrefSize};
use crate::widget_graph::WidgetContext;
use crate::event::Event;
use druid_shell::piet::{Piet, RenderContext};
use crate::state::key::Key;

struct Child<W> {
    widget: W,
    offset: Vec2,
    size: Size,
}

/// The Layout of a Container
/// the display order is back to front
pub trait Layout<T: Clone, W: Widget<T>> {
    type Constrain;
    type Meta;

    fn insert(&mut self, widget: W, constrain: Self::Constrain) -> u32;
    fn calc_pref_size(&self, context: WidgetContext, data: &T) -> PrefSize;
    fn layout(&mut self, size: Size, context: WidgetContext, data: &T);
    fn get_widgets(&self) -> &[(Child<W>, Self::Meta)];
    fn get_widgets_mut(&mut self) -> &mut [(Child<W>, Self::Meta)];

}

pub trait ListLayout<T: Clone, W: Widget<T>>: Layout<T, W> + Default {
    fn append(&mut self, widget: W) -> u32;
    fn remove(&mut self, index: u32);
}

pub struct Container<L> {
    layout: L,
    mouse_focus: Option<u32>,
    focus: Option<u32>,
}

impl<T, W: Widget<T>, L: Layout<T, W>> Container<L> {
    pub fn new(layout: L) -> Self {
        Container {
            layout,
            mouse_focus: None,
            focus: None,
        }
    }
    pub fn with(mut self, constrain: L::Constrain, widget: W) -> Self {
        self.layout.insert(widget, constrain);
        self
    }
}

impl<T, W: Widget<T>, L: Layout<T, W>> Widget<T> for Container<L> {
    fn draw(&self, painter: &mut Piet, dirty_rect: Rect, context: WidgetContext, data: &T) {
        for (child, _) in self.layout.get_widgets() {
            painter.with_save(|painter|{
                painter.transform(Affine::translate(child.offset));
                child.widget.draw(painter, dirty_rect, context.id(), data);//TODO: translate dirty rect!
                Ok(())
            });
        }
    }

    fn handle_event(&mut self, event: Event, context: WidgetContext, data: Key<T>) {
        //TODO: implement
    }

    fn get_pref_size(&self, context: WidgetContext, data: &T) -> PrefSize {
        self.layout.calc_pref_size(context, data)
    }

    fn layout(&mut self, size: Size, context: WidgetContext, data: &T) {
        self.layout.layout(size, context, data);
    }

    fn build(&mut self, mut context: WidgetContext) {
        for (child, _) in self.layout.get_widgets_mut() {
            child.widget.build(context.id())
        }
    }
}