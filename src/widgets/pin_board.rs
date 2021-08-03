use std::fmt::Debug;

use druid::{
    Point, WidgetPod
};
use druid::widget::*;
use druid::widget::prelude::*;

use super::canvas::{Canvas, CanvasData, Positioned, Identifiable};
use crate::controllers::RecordUndoStateExt;

pub trait Pinnable : Positioned + Identifiable {
    fn new(position: Point, id: u64) -> Self;
}

pub trait PinBoardData<C> : Positioned + CanvasData<C> {
    fn add_pin(&mut self, new_pin: C);
    fn remove_pin(&mut self, pin_id: &u64);
    fn pins(&self) -> Box<dyn Iterator<Item=&C> + '_>;
}

pub struct PinBoard<C, D, W> {
    pub canvas: WidgetPod<D, Canvas<C, D, W>>,

    pub mouse_down_position: Option<Point>,
    pub pin_id_under_mouse: Option<u64>,
}

impl<C: Data + Pinnable + PartialEq, D: CanvasData<C> + PinBoardData<C>, W: Widget<C>> PinBoard<C, D, W> {
    pub fn new(
        new_widget: impl Fn() -> W + 'static,
    ) -> PinBoard<C, D, W> {
        let canvas = Canvas::new(new_widget);
        PinBoard {
            canvas: WidgetPod::new(canvas),

            mouse_down_position: None,
            pin_id_under_mouse: None,
        }
    }

    pub fn canvas(&self) -> &Canvas<C, D, W> {
        self.canvas.widget()
    }

    pub fn canvas_mut(&mut self) -> &mut Canvas<C, D, W> {
        self.canvas.widget_mut()
    }

    pub fn new_pin(&self, position: Point, data: &D) -> (u64, C) {
        let highest_pin_id = data.children().map(|(id, _)| id).max().unwrap_or(&0);

        let offset_position = (position.to_vec2() - data.get_position().to_vec2()).to_point();
        let pin_id = highest_pin_id + 1;
        (pin_id, C::new(offset_position, pin_id))
    } 

    fn add_pin(&mut self, position: Point, data: &mut D) {
        let (pin_id, new_pin) = self.new_pin(position, data);
        data.add_pin(new_pin);
    }
}

impl<C: Data + Debug + Pinnable + PartialEq, D: Data + Positioned + CanvasData<C> + PinBoardData<C>, W: Widget<C>> Widget<D> for PinBoard<C, D, W> {
    fn event(&mut self, ctx: &mut EventCtx, ev: &Event, data: &mut D, env: &Env) {
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
                for (child_id, child_data) in data.children() {
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
                            data.remove_pin(pin_id_under_mouse);
                            ctx.record_undo_state();
                        }
                    }
                }

                self.mouse_down_position = None;
            },
            _ => {}
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, ev: &LifeCycle, data: &D, env: &Env) {
        self.canvas.lifecycle(ctx, ev, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &D, data: &D, env: &Env) {
        self.canvas.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &D, env: &Env) -> Size {
        self.canvas.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &D, env: &Env) {
        self.canvas.paint(ctx, data, env);
    }
}
