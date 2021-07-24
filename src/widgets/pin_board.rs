use std::fmt::Debug;

use druid::{
    Point, WidgetPod
};
use druid::im::Vector;
use druid::widget::*;
use druid::widget::prelude::*;

use super::canvas::{Canvas, Positioned, Identifiable};
use crate::controllers::RecordUndoStateExt;

pub trait Pinnable : Positioned + Identifiable {
    fn new(position: Point, id: u64) -> Self;
}

pub struct PinBoard<C, W> {
    pub canvas: WidgetPod<(Point, Vector<C>), Canvas<C, W>>,

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

    pub fn new_pin(&mut self, position: Point, data: &(Point, Vector<C>)) -> C {
        let (offset, children) = data;
        let mut highest_pin_id = 0;
        for child in children {
            if child.get_id() > highest_pin_id {
                highest_pin_id = child.get_id();
            }
        }

        let offset_position = (position.to_vec2() - offset.to_vec2()).to_point();
        let pin_id = highest_pin_id + 1;
        C::new(offset_position, pin_id)
    } 

    fn add_pin(&mut self, position: Point, data: &mut(Point, Vector<C>)) {
        let new_pin = self.new_pin(position, data);
        let (_, child_data_vector) = data;
        child_data_vector.push_back(new_pin);
    }
}

impl<C: Data + Debug + Pinnable + PartialEq, W: Widget<C>> Widget<(Point, Vector<C>)> for PinBoard<C, W> {
    fn event(&mut self, ctx: &mut EventCtx, ev: &Event, data: &mut (Point, Vector<C>), env: &Env) {
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
                for child_data in data.1.iter() {
                    let child_id = child_data.get_id();
                    if let Some(child_location) = self.canvas.widget().get_child_position(&child_id) {
                        if child_location.contains(mouse_event.pos) {
                            self.pin_id_under_mouse = Some(child_id);
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
                            let (_, child_data_vector) = data;
                            child_data_vector.retain(|child_data| &child_data.get_id() != pin_id_under_mouse);
                            ctx.record_undo_state();
                        }
                    }
                }

                self.mouse_down_position = None;
            },
            _ => {}
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, ev: &LifeCycle, data: &(Point, Vector<C>), env: &Env) {
        self.canvas.lifecycle(ctx, ev, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &(Point, Vector<C>), data: &(Point, Vector<C>), env: &Env) {
        self.canvas.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &(Point, Vector<C>), env: &Env) -> Size {
        self.canvas.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &(Point, Vector<C>), env: &Env) {
        self.canvas.paint(ctx, data, env);
    }
}
