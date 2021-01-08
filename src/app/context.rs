use druid_shell::piet::{Piet, RenderContext};
use druid_shell::kurbo::{Vec2, Rect, Affine, Size, Point};
use std::marker::PhantomData;
use crate::app::event::Event;

fn translate_rect(rect: Rect, data: Vec2) -> Rect {
    Rect::new(rect.x0 + data.x, rect.y0 + data.y, rect.x1 + data.x, rect.y1 + data.y)
}

pub struct WidgetContext<'a> {
    ///The widget to Window transform of this widget
    transform: Affine,
    size: Size,
    a: PhantomData<&'a mut ()>,
}



impl<'a> WidgetContext<'a> {
    /// translates the transform to a child.
    ///
    /// translation is the Vector from the parent origin to the child origin
    pub fn child(&mut self, translation: Vec2, child_size: Size) -> WidgetContext {
        //This is the translation back therefore we multiply in this direction
        //
        //DrawContext::translate uses self.translation *= translation:
        //
        //Draw translation: A * B * C * D WidgetContext: D^-1 * C^-1 * B^-1 * A^-1
        //   A * B * C * D * D^-1 * C^-1 * B^-1 * A^-1
        // = A * B * C * ID * C^-1 * B^-1 * A^-1
        // = A * B * C * C^-1 * B^-1 * A^-1
        // = A * B * ID * B^-1 * A^-1
        //...
        let mut transform = Affine::translate(-translation) * self.transform;

        WidgetContext {
            transform,
            size: child_size,
            a: Default::default(),
        }
    }
    /// applies a parent-to-child transform to the Context
    ///
    /// for translations use the much cheaper translate method
    pub fn apply_transform(&mut self, transform: Affine) -> WidgetContext {
        WidgetContext {
            transform: transform * self.transform,
            size: self.size,
            a: Default::default(),
        }
    }
    pub fn widget_to_window_transform(&self) -> Affine {
        self.transform
    }
    pub fn window_to_widget_transform(&self) -> Affine {
        self.transform.inverse()
    }
    pub fn size(&self) -> Size {
        self.size
    }
    pub fn bbox(&self) -> Rect {
        self.size.to_rect()
    }
    pub fn global_bbox(&self) -> Rect {
        self.widget_to_window_transform().transform_rect_bbox(self.bbox())
    }
    pub fn contains(&self, point: Point) -> bool {
        self.bbox().contains(point)
    }
    pub fn contains_global(&self, global_point: Point) -> bool {
        self.contains(self.window_to_widget_transform() * global_point)
    }
}

macro_rules! forward_widget_context{
    ($impl_type:ty) => {
        impl<'a, 'b> $impl_type {
            pub fn widget_to_window_transform(&self) -> Affine {
                self.widget_ctx.widget_to_window_transform()
            }
            pub fn window_to_widget_transform(&self) -> Affine {
                self.widget_ctx.window_to_widget_transform()
            }
            pub fn size(&self) -> Size {
                self.widget_ctx.size()
            }
            pub fn bbox(&self) -> Rect {
                self.widget_ctx.bbox()
            }
            pub fn global_bbox(&self) -> Rect {
                self.widget_ctx.global_bbox()
            }
            pub fn contains(&self, point: Point) -> bool {
                self.widget_ctx.contains(point)
            }
            pub fn contains_global(&self, global_point: Point) -> bool {
                self.widget_ctx.contains_global(global_point)
            }
        }
    }
}

pub struct PaintContext<'a, 'b> {
    widget_ctx: WidgetContext<'a>,
    piet: &'a mut Piet<'b>,
    dirty_rect: Rect,
}

forward_widget_context!(PaintContext<'a, 'b>);

impl<'a, 'b> PaintContext<'a, 'b> {
    fn new(widget_ctx: WidgetContext<'a>, piet: &'a mut Piet<'b>, dirty_rect: Rect) -> Self {
        PaintContext {
            widget_ctx,
            piet,
            dirty_rect,
        }
    }
    pub fn as_child<R>(&mut self, translation: Vec2, new_size: Size, run: impl FnOnce(PaintContext) -> R) -> R {
        let mut value = None;
        let Self{piet, widget_ctx, dirty_rect} = self;
        let error = piet.with_save(|piet|{
            piet.transform(Affine::translate(translation));
            let ctx = widget_ctx.child(translation, new_size);
            let dirty_rect = translate_rect(*dirty_rect, -translation);
            value = Some(run(PaintContext::new(ctx, piet, dirty_rect)));
            Ok(())
        });
        if let Err(e) = error {
            eprintln!("{:?}", e);
        }
        value.expect("piet with save errored!")
    }
}

pub struct LayoutContext<'a> {
    widget_ctx: WidgetContext<'a>,
    allocated_size: Size,
}

impl<'a> LayoutContext<'a> {
    pub fn new(widget_ctx: WidgetContext<'a>, allocated_size: Size) -> Self {
        LayoutContext {
            widget_ctx,
            allocated_size,
        }
    }
    pub fn as_child<R>(&mut self, translation: Vec2, new_size: Size, run: impl FnOnce(LayoutContext) -> R) -> R{
        let ctx = self.widget_ctx.child(translation, new_size);

        run(LayoutContext::new(ctx, new_size))
    }
    pub fn widget_to_window_transform(&self) -> Affine {
        self.widget_ctx.widget_to_window_transform()
    }
    pub fn window_to_widget_transform(&self) -> Affine {
        self.widget_ctx.window_to_widget_transform()
    }
    pub fn allocated_size(&self) -> Size {
        self.allocated_size
    }
    pub fn with_aspect_ratio(&self, preferred_aspect_ratio: f64) -> Size {
        let current_ratio = self.allocated_size.width / self.allocated_size.height;

        if current_ratio < preferred_aspect_ratio {
            //X
            Size::new(self.allocated_size.width, self.allocated_size.width / preferred_aspect_ratio)
        } else {
            Size::new(self.allocated_size.height * preferred_aspect_ratio, self.allocated_size.height)
        }
    }
}

/// Widget Change context contains the methods for signaling change present in UpdateContext
/// and EventContext
pub trait WidgetChangeContext<'a> {
    fn request_layout(&mut self);
    fn repaint(&mut self);
    fn repaint_area(&mut self, area: Rect);
    fn widget_context(&self) -> WidgetContext<'a>;
}

pub struct EventContext<'a> {
    widget_ctx: WidgetContext<'a>,
    event: Event,
    global_dirty_rect: &'a mut Option<Rect>,
    need_layout: &'a mut bool,
}

forward_widget_context!(EventContext<'a>);

impl<'a> EventContext<'a> {
    pub fn new(widget_ctx: WidgetContext<'a>, event: Event, global_dirty_rect: &'a mut Option<Rect>, need_layout: &'a mut bool) -> Self {
        EventContext {
            widget_ctx,
            event,
            global_dirty_rect,
            need_layout,
        }
    }
    pub fn event(&self) -> &Event {
        &self.event
    }
    pub fn request_layout(&mut self) {
        *self.need_layout = true;
    }
    pub fn request_redraw_area(&mut self, area: Rect) {
        let bbox = self.widget_to_window_transform().transform_rect_bbox(area);
        let new_dirty_rect = match *self.global_dirty_rect {
            Some(old) => old.union(bbox),
            None => bbox,
        };
        *self.global_dirty_rect = Some(new_dirty_rect);
    }

    pub fn request_redraw(&mut self) {
        let area = self.bbox();
        self.request_redraw_area(area);
    }

    pub fn as_child<R>(&mut self, translation: Vec2, new_size: Size, run: impl FnOnce(EventContext) -> R) -> Option<R> {
        let ctx = self.widget_ctx.child(translation, new_size);
        let mut event = self.event.clone();
        let fit = event.shift(translation, new_size);
        if fit {
            let mut child_need_layout = false;

            let ret = run(EventContext::new(ctx, event, &mut self.global_dirty_rect, &mut child_need_layout));

            //TODO: handle child handle layout
            *self.need_layout |= child_need_layout;

            Some(ret)
        } else {
            None
        }
    }
}

pub struct UpdateContext<'a> {
    widget_ctx: WidgetContext<'a>,
    global_dirty_rect: &'a mut Rect,
    need_layout: bool,
}