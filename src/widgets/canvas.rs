use std::cmp::Ordering;
use std::collections::HashMap;

use druid::{
    Point, WidgetPod, Rect
};
use druid::im::Vector;
use druid::widget::ListIter;
use druid::widget::prelude::*;

use crate::controllers::draggable::Positioned;
use super::pinboard::Pinnable;

// Widget which renders it's children on an infinite grid
pub struct Canvas<C> {
    new_widget_closure: Box<dyn Fn() -> Box<dyn Widget<C>>>,
    children: Vec<WidgetPod<C, Box<dyn Widget<C>>>>,
    child_positions: HashMap<String, Rect>,
}

impl<C: Data + Positioned + Pinnable> Canvas<C> {
    pub fn new<W: Widget<C> + 'static>(new_widget_closure: impl Fn() -> W + 'static) -> Self {
        Canvas {
            new_widget_closure: Box::new(move || Box::new(new_widget_closure())),
            children: Vec::new(),
            child_positions: HashMap::new(),
        }
    }

    fn update_child_count(&mut self, data: &Vector<C>, _env: &Env) -> bool {
        let len = self.children.len();
        match len.cmp(&data.data_len()) {
            Ordering::Greater => self.children.truncate(data.data_len()),
            Ordering::Less => data.for_each(|_, i| {
                if i >= len {
                    let child = WidgetPod::new((self.new_widget_closure)());
                    self.children.push(child);
                }
            }),
            Ordering::Equal => (),
        }
        len != data.data_len()
    }

    pub fn get_child_position(&self, id: &String) -> Option<&Rect> {
        self.child_positions.get(id)
    }
}

impl<C: Data + Positioned + Pinnable> Widget<(Point, Vector<C>)> for Canvas<C> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut (Point, Vector<C>), env: &Env) {
        let mut children = self.children.iter_mut();

        let (_, data_list) = data;
        data_list.for_each_mut(|child_data, _| {
            if let Some(child) = children.next() {
                child.event(ctx, event, child_data, env);
            }
        });
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &(Point, Vector<C>), env: &Env) {
        let (_, data_list) = data;
        if let LifeCycle::WidgetAdded = event {
            if self.update_child_count(data_list, env) {
                ctx.children_changed();
            }
        }

        let mut children = self.children.iter_mut();
        data_list.for_each(|child_data, _| {
            if let Some(child) = children.next() {
                child.lifecycle(ctx, event, child_data, env);
            }
        });
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &(Point, Vector<C>), data: &(Point, Vector<C>), env: &Env) {
        let (_, data_list) = data;
        // we send update to children first, before adding or removing children;
        // this way we avoid sending update to newly added children, at the cost
        // of potentially updating children that are going to be removed.
        let mut children = self.children.iter_mut();
        data_list.for_each(|child_data, _| {
            if let Some(child) = children.next() {
                child.update(ctx, child_data, env);
            }
        });

        if self.update_child_count(data_list, env) {
            ctx.children_changed();
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &(Point, Vector<C>), env: &Env) -> Size {
        let (offset, data_list) = data;

        let mut new_child_positions = HashMap::new();
        let mut children = self.children.iter_mut();
        let child_bc = BoxConstraints::UNBOUNDED;
        data_list.for_each(|child_data, _| {
            let child = match children.next() {
                Some(child) => child,
                None => {
                    return;
                }
            };

            let child_size = child.layout(ctx, &child_bc, child_data, env);
            let child_position = child_data.get_position();
            let offset_position = Point::new(child_position.x + offset.x, child_position.y + offset.y);
            new_child_positions.insert(child_data.get_id(), Rect::from_origin_size(offset_position, child_size));
            child.set_origin(ctx, child_data, env, offset_position);
        });

        self.child_positions = new_child_positions;
        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &(Point, Vector<C>), env: &Env) {
        let (_, data_list) = data;
        let mut children = self.children.iter_mut();
        data_list.for_each(|child_data, _| {
            if let Some(child) = children.next() {
                child.paint(ctx, child_data, env);
            }
        });
    }
}
