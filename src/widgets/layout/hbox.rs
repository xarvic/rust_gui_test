use crate::widgets::layout::{Child, Layout};
use crate::widget_graph::WidgetContext;
use crate::widgets::{PrefSize, Widget};
use druid_shell::kurbo::{Size, Vec2};

#[derive(Default)]
pub struct HBox<W>{
    childs: Vec<(Child<W>, ())>,
}

impl<W> HBox<W> {
    pub fn new() -> Self {
        Default::default()
    }
}

impl<T: Clone, W: Widget<T>> Layout<T, W> for HBox<W> {
    type Constrain = u32;
    type Meta = ();

    fn insert(&mut self, widget: W, mut constrain: u32) -> u32 {
        constrain = constrain.min(self.childs.len() as u32);
        self.childs.insert(constrain as usize, widget);

        constrain
    }

    fn calc_pref_size(&self, context: WidgetContext, data: &T) -> PrefSize {
        let mut pref = PrefSize::zero();

        for (child, _) in self.childs.iter() {
            pref.row(child.widget.get_pref_size(context.id(), data));
        }

        pref
    }

    fn layout(&mut self, size: Size, context: WidgetContext, data: &T) {
        let pref_size = self.calc_pref_size(context.id(), data);

        let rel = (size.width.max(pref_size.min.width)
            .min(pref_size.max.width)
            - pref_size.min.width) / (pref_size.max.width - pref_size.max.width);
        let add = (size.width.min(pref_size.max.width) - pref_size.max.width) /
            self.childs.len();

        let mut advance = 0.0;

        for (child, _) in self.get_widgets_mut() {
            let child_pref = child.widget.get_pref_size(context.id(), data);
            let child_size =
                Size::new(child_pref.min.width * rel +
                              child_pref.max.width * (1.0 - rel)
                          + add, size.height);

            child.widget.layout(child_size, context.id(), data);
            child.size = child_size;
            child.offset = Vec2::new(advance, 0.0);
            advance += child_size.width;
        }

    }

    fn get_widgets(&self) -> &[(Child<W>, Self::Meta)] {
        &self.childs
    }

    fn get_widgets_mut(&mut self) -> &mut [(Child<W>, Self::Meta)] {
        &mut self.childs
    }
}