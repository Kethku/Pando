use std::fmt::Debug;

use druid::{
    Point, WidgetPod
};
use druid::im::HashMap as ImHashMap;
use druid::widget::*;
use druid::widget::prelude::*;

use super::canvas::{Canvas, Positioned, Identifiable};
use crate::controllers::RecordUndoStateExt;

pub trait Pinnable : Positioned + Identifiable {
    fn new(position: Point, id: u64) -> Self;
}

pub struct PinBoard<C, W> {
    pub canvas: WidgetPod<(Point, ImHashMap<u64, C>), Canvas<C, W>>,

    pub mouse_down_position: Option<Point>,
    pub pin_id_under_mouse: Option<u64>,
}

impl<C: Data + Pinnable + PartialEq, W: Widget<C>> PinBoard<C, W> {
    pub fn new(
        new_widget: impl Fn() -> W + 'static,
    ) -> PinBoard<C, W> {
        let canvas = Canvas::new(new_widget);
        PinBoard {
            canvas: WidgetPod::new(canvas),

            mouse_down_position: None,
            pin_id_under_mouse: None,
        }
    }

    pub fn canvas(&self) -> &Canvas<C, W> {
        self.canvas.widget()
    }

    pub fn canvas_mut(&mut self) -> &mut Canvas<C,W> {
        self.canvas.widget_mut()
    }

    pub fn new_pin(&mut self, position: Point, data: &(Point, ImHashMap<u64, C>)) -> (u64, C) {
        let (offset, children) = data;
        let highest_pin_id = children.keys().max().unwrap_or(&0);

        let offset_position = (position.to_vec2() - offset.to_vec2()).to_point();
        let pin_id = highest_pin_id + 1;
        (pin_id, C::new(offset_position, pin_id))
    } 

    fn add_pin(&mut self, position: Point, data: &mut(Point, ImHashMap<u64, C>)) {
        let (pin_id, new_pin) = self.new_pin(position, data);
        let (_, child_data_map) = data;
        child_data_map.insert(pin_id, new_pin);
    }
}

impl<C: Data + Debug + Pinnable + PartialEq, W: Widget<C>> Widget<(Point, ImHashMap<u64, C>)> for PinBoard<C, W> {
    fn event(&mut self, ctx: &mut EventCtx, ev: &Event, data: &mut (Point, ImHashMap<u64, C>), env: &Env) {
        self.canvas.event(ctx, ev, data, env);

        if ctx.is_handled() {
            return;
        }

        match ev {
            Event::MouseDown(mouse_event) => {
                self.mouse_down_position = Some(mouse_event.pos);
            },
            Event::MouseMove(mouse_event) => {
                self.pin_id_under_mouse = None;
                for (child_id, child_data) in data.1.iter() {
                    if let Some(child_location) = self.canvas.widget().get_child_position(&child_id) {
                        if child_location.contains(mouse_event.pos) {
                            self.pin_id_under_mouse = Some(*child_id);
                        }
                    }
                }
            },
            Event::MouseUp(mouse_event) => {
                if let Some(mouse_down_position) = self.mouse_down_position {
                    if mouse_event.button.is_left() && mouse_event.pos == mouse_down_position {
                        self.add_pin(mouse_down_position, data);
                    } else if mouse_event.button.is_right() {
                        if let Some(pin_id_under_mouse) = &self.pin_id_under_mouse {
                            let (_, child_data_map) = data;
                            child_data_map.remove(pin_id_under_mouse);
                            ctx.record_undo_state();
                        }
                    }
                }

                self.mouse_down_position = None;
            },
            _ => {}
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, ev: &LifeCycle, data: &(Point, ImHashMap<u64, C>), env: &Env) {
        self.canvas.lifecycle(ctx, ev, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &(Point, ImHashMap<u64, C>), data: &(Point, ImHashMap<u64, C>), env: &Env) {
        self.canvas.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &(Point, ImHashMap<u64, C>), env: &Env) -> Size {
        self.canvas.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &(Point, ImHashMap<u64, C>), env: &Env) {
        self.canvas.paint(ctx, data, env);
    }
}
