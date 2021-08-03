use std::collections::HashMap;
use std::marker::PhantomData;

use druid::{
    Point, Rect, WidgetPod, 
};
use druid::widget::prelude::*;

pub trait Positioned {
    fn get_position(&self) -> Point;
    fn set_position(&mut self, new_position: Point);

    fn get_top_left_position(&self, size: Size) -> Point {
        let position = self.get_position();
        position - (size.to_vec2() / 2.0)
    }
}

impl Positioned for Point {
    fn get_position(&self) -> Point {
        *self
    }

    fn set_position(&mut self, new_position: Point) {
        *self = new_position
    }
}

impl<T> Positioned for (Point, T) {
    fn get_position(&self) -> Point {
        self.0
    }

    fn set_position(&mut self, new_position: Point) {
        let (position, _) = self;
        *position = new_position
    }
}

pub trait Identifiable {
    fn get_id(&self) -> u64;
}

pub trait CanvasData<C> {
    fn children(&self) -> Box<dyn Iterator<Item=(&u64, &C)> + '_>;
    fn children_mut(&mut self) -> Box<dyn Iterator<Item=(&u64, &mut C)> + '_>;
    fn get_child(&self, id: &u64) -> Option<&C>;
    fn get_child_mut(&mut self, id: &u64) -> Option<&mut C>;
}

// Widget which renders it's children on an infinite grid
pub struct Canvas<C, D, W> {
    phantom: PhantomData<D>,
    new_widget_closure: Box<dyn Fn() -> W>,
    children: HashMap<u64, WidgetPod<C, W>>,
    child_positions: HashMap<u64, Rect>,
}

impl<C: Data + Positioned + PartialEq, D: CanvasData<C>, W: Widget<C>> Canvas<C, D, W> {
    pub fn new(new_widget_closure: impl Fn() -> W + 'static) -> Self {
        Canvas {
            phantom: PhantomData,
            new_widget_closure: Box::new(new_widget_closure),
            children: HashMap::new(),
            child_positions: HashMap::new(),
        }
    }

    fn update_widgets(&mut self, data: &D, _env: &Env) -> bool {
        let mut child_ids_to_remove = Vec::new();
        let mut child_ids_to_add = Vec::new();

        for widget_child_id in self.children.keys().copied() {
            if !data.get_child(&widget_child_id).is_some() {
                child_ids_to_remove.push(widget_child_id);
            }
        }
        for (data_child_id, _) in data.children() {
            if !self.children.contains_key(&data_child_id) {
                child_ids_to_add.push(*data_child_id);
            }
        }

        let children_changed = !child_ids_to_remove.is_empty() || !child_ids_to_add.is_empty();

        for child_id_to_remove in child_ids_to_remove.into_iter() {
            self.children.remove(&child_id_to_remove);
        }
        for child_id_to_add in child_ids_to_add.into_iter() {
            self.children.insert(child_id_to_add, WidgetPod::new((self.new_widget_closure)()));
        }

        children_changed
    }

    pub fn get_child_position(&self, child_id: &u64) -> Option<&Rect> {
        self.child_positions.get(child_id)
    }
}

impl<C: Data + Positioned + Identifiable + PartialEq, D: Positioned + CanvasData<C>, W: Widget<C>> Widget<D> for Canvas<C, D, W> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut D, env: &Env) {
        for (id, child_data) in data.children_mut() {
            if let Some(child_widget) = self.children.get_mut(&id) {
                child_widget.event(ctx, event, child_data, env);
            }
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &D, env: &Env) {
        if let LifeCycle::WidgetAdded = event {
            if self.update_widgets(data, env) {
                ctx.children_changed();
            }
        }

        for (id, child_data) in data.children() {
            if let Some(child_widget) = self.children.get_mut(&id) {
                child_widget.lifecycle(ctx, event, child_data, env);
            }
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &D, data: &D, env: &Env) {
        // we send update to children first, before adding or removing children;
        // this way we avoid sending update to newly added children, at the cost
        // of potentially updating children that are going to be removed.
        for (id, child_data) in data.children() {
            if let Some(child_widget) = self.children.get_mut(&id) {
                child_widget.update(ctx, child_data, env);
            }
        }

        if self.update_widgets(data, env) {
            ctx.children_changed();
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &D, env: &Env) -> Size {
        let mut new_child_positions = HashMap::new();
        let child_bc = BoxConstraints::UNBOUNDED;

        for (id, child_data) in data.children() {
            if let Some(child_widget) = self.children.get_mut(&id) {
                let child_size = child_widget.layout(ctx, &child_bc, child_data, env);
                let child_top_left = child_data.get_top_left_position(child_size);
                let offset_position = child_top_left + data.get_position().to_vec2();
                new_child_positions.insert(child_data.get_id(), Rect::from_origin_size(offset_position, child_size));
                child_widget.set_origin(ctx, child_data, env, offset_position);
            } 
        }

        self.child_positions = new_child_positions;
        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &D, env: &Env) {
        for (id, child_data) in data.children() {
            if let Some(child_widget) = self.children.get_mut(&id) {
                child_widget.paint(ctx, child_data, env);
            } 
        }
    }
}
