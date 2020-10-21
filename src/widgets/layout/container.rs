use crate::widgets::layout::{WidgetList, ChildMeta, Layout};
use crate::widgets::{Widget, PrefSize};
use crate::event::Event;
use crate::widget_graph::WidgetContext;
use crate::state::key::Key;
use druid_shell::kurbo::{Size, Affine, Rect};
use druid_shell::piet::{Piet, RenderContext, Color};

type List<T, Meta> = Vec<(Box<dyn Widget<T>>, ChildMeta<Meta>)>;

impl<T: Clone, Meta> WidgetList<Meta> for List<T, Meta> {
    fn iter_inner(&self, mut iterator: impl FnMut(&ChildMeta<Meta>)) {
        self.iter().for_each(|(_, meta)|iterator(meta));
    }

    fn iter_inner_mut(&mut self, mut iterator: impl FnMut(&mut ChildMeta<Meta>)) {
        self.iter_mut().for_each(|(_, meta)|iterator(meta));
    }

    fn get_with(&self, index: usize, mut iterator: impl FnMut(&ChildMeta<Meta>)) {
        self.get(index).map(|(_, meta)|iterator(meta));
    }

    fn get_mut_with(&mut self, index: usize, mut iterator: impl FnMut(&mut ChildMeta<Meta>)) {
        self.get_mut(index).map(|(_, meta)|iterator(meta));
    }

    fn count(&self) -> u32 {
        self.len() as u32
    }
}

pub struct Container<T: Clone, L: Layout> {
    widgets: List<T, L::Meta>,
    layout: L,
    mouse_focus: Option<u32>,
    focus: Option<u32>,
}

impl<T: Clone, L: Layout> Container<T, L> {
    pub fn new(layout: L) -> Self {
        Container {
            widgets: Vec::new(),
            layout,
            mouse_focus: None,
            focus: None,
        }
    }
    pub fn constrained_child(mut self, constrain: L::Constrain, widget: impl Widget<T> + 'static) -> Self {
        let (index, meta) = self.layout.insert(constrain);
        self.widgets.insert(index as usize, (Box::new(widget), ChildMeta::empty(meta)));

        self
    }
}

impl<T: Clone, L: Layout> Container<T, L> where L::Constrain: Default {
    pub fn child(self, widget: impl Widget<T> + 'static) -> Self {
        self.constrained_child(Default::default(), widget)
    }
}

impl<T: Clone + 'static, L: Layout> Widget<T> for Container<T, L> {
    fn draw(&mut self, painter: &mut Piet, size: Size, dirty_rect: Rect, context: WidgetContext, data: &T) {
        let brush = painter.solid_brush(Color::rgb8(185, 0, 0));
        painter.stroke(Rect::new(1.0, 1.0, size.width - 1.0, size.height - 1.0), &brush, 2.0);
        for (child, meta) in self.widgets.iter_mut() {
            painter.with_save(|painter|{
                painter.transform(Affine::translate(meta.offset));
                child.draw(painter, meta.size, dirty_rect, context.id(), data);//TODO: translate dirty rect!
                Ok(())
            }).unwrap();
        }
    }

    fn handle_event(&mut self, mut event: Event, context: WidgetContext, mut data: Key<T>) {
        if let Some(me) = event.mouse_event() {
            let mut consumer = None;
            for (index, (child, meta)) in self.widgets.iter().enumerate() {
                if Rect::from_origin_size((meta.offset.x, meta.offset.y), meta.size).contains(me.pos) {
                    consumer = Some(index as u32);
                }
            }

            //Exit old widget
            if let Some(old) = self.mouse_focus {
                if consumer.unwrap_or(old + 1) != old {
                    self.widgets[old as usize].0.handle_event(Event::MouseExit, context.id(), data.id());
                }
            }
            //Enter new widget
            if let Some(new) = consumer {
                let (child, meta) = &mut self.widgets[new as usize];
                let mut child_event = me.clone();
                child_event.pos -= meta.offset;

                if self.mouse_focus.unwrap_or(new + 1) != new {
                    child.handle_event(Event::MouseEnter(child_event.clone()), context.id(), data.id());
                }
                if let Event::MouseEnter(_) = event {
                    //Do nothing (already done)!
                } else {
                    event.shift(meta.offset);
                    child.handle_event(event, context.id(), data.id());
                }
            }
            self.mouse_focus = consumer;
        } else {

        }

    }

    fn get_pref_size(&mut self, context: WidgetContext, data: &T) -> PrefSize {
        for (child, meta) in self.widgets.iter_mut() {
            meta.pref = child.get_pref_size(context.id(), data);
        }
        self.layout.calc_pref_size(&self.widgets)
    }

    fn layout(&mut self, size: Size, context: WidgetContext, data: &T) {
        self.layout.layout(size, &mut self.widgets);
        for (child, meta) in self.widgets.iter_mut() {
            child.layout(meta.size, context.id(), data);
        }
    }

    fn build(&mut self, context: WidgetContext) {
        for (child, _) in self.widgets.iter_mut() {
            child.build(context.id())
        }
    }
}