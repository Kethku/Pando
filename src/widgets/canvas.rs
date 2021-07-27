use std::cmp::Ordering;
use std::collections::HashMap;

use druid::{
    Point, Rect, WidgetPod, 
};
use druid::im::HashMap as ImHashMap;
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

// Widget which renders it's children on an infinite grid
pub struct Canvas<C, W> {
    new_widget_closure: Box<dyn Fn() -> W>,
    children: HashMap<u64, WidgetPod<C, W>>,
    child_positions: HashMap<u64, Rect>,
}

impl<C: Data + Positioned + PartialEq, W: Widget<C>> Canvas<C, W> {
    pub fn new(new_widget_closure: impl Fn() -> W + 'static) -> Self {
        Canvas {
            new_widget_closure: Box::new(new_widget_closure),
            children: HashMap::new(),
            child_positions: HashMap::new(),
        }
    }

    fn update_widgets(&mut self, data: &ImHashMap<u64, C>, _env: &Env) -> bool {
        let mut child_ids_to_remove = Vec::new();
        let mut child_ids_to_add = Vec::new();

        for widget_child_id in self.children.keys().copied() {
            if !data.contains_key(&widget_child_id) {
                child_ids_to_remove.push(widget_child_id);
            }
        }
        for data_child_id in data.keys().copied() {
            if !self.children.contains_key(&data_child_id) {
                child_ids_to_add.push(data_child_id);
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

impl<C: Data + Positioned + Identifiable + PartialEq, W: Widget<C>> Widget<(Point, ImHashMap<u64, C>)> for Canvas<C, W> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut (Point, ImHashMap<u64, C>), env: &Env) {
        let (_, data_map) = data;

        for (id, child_data) in data_map.iter_mut() {
            if let Some(child_widget) = self.children.get_mut(&id) {
                child_widget.event(ctx, event, child_data, env);
            }
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &(Point, ImHashMap<u64, C>), env: &Env) {
        let (_, data_map) = data;
        if let LifeCycle::WidgetAdded = event {
            if self.update_widgets(data_map, env) {
                ctx.children_changed();
            }
        }

        for (id, child_data) in data_map {
            if let Some(child_widget) = self.children.get_mut(&id) {
                child_widget.lifecycle(ctx, event, child_data, env);
            }
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &(Point, ImHashMap<u64, C>), data: &(Point, ImHashMap<u64, C>), env: &Env) {
        let (_, data_map) = data;
        // we send update to children first, before adding or removing children;
        // this way we avoid sending update to newly added children, at the cost
        // of potentially updating children that are going to be removed.
        for (id, child_data) in data_map {
            if let Some(child_widget) = self.children.get_mut(&id) {
                child_widget.update(ctx, child_data, env);
            }
        }

        if self.update_widgets(data_map, env) {
            ctx.children_changed();
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &(Point, ImHashMap<u64, C>), env: &Env) -> Size {
        let (offset, data_map) = data;

        let mut new_child_positions = HashMap::new();
        let child_bc = BoxConstraints::UNBOUNDED;

        for (id, child_data) in data_map {
            if let Some(child_widget) = self.children.get_mut(&id) {
                let child_size = child_widget.layout(ctx, &child_bc, child_data, env);
                let child_top_left = child_data.get_top_left_position(child_size);
                let offset_position = child_top_left + offset.to_vec2();
                new_child_positions.insert(child_data.get_id(), Rect::from_origin_size(offset_position, child_size));
                child_widget.set_origin(ctx, child_data, env, offset_position);
            } 
        }

        self.child_positions = new_child_positions;
        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &(Point, ImHashMap<u64, C>), env: &Env) {
        let (_, data_map) = data;

        for (id, child_data) in data_map {
            if let Some(child_widget) = self.children.get_mut(&id) {
                child_widget.paint(ctx, child_data, env);
            } 
        }
    }
}
