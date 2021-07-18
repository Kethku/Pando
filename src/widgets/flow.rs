use std::fmt::Debug;

use druid::{
    Point, WidgetPod, RenderContext, Vec2, theme, Rect
};
use druid::im::Vector;
use druid::kurbo::CubicBez;
use druid::widget::*;
use druid::widget::prelude::*;

use super::pin_board::{PinBoard, Pinnable};
use crate::controllers::RecordUndoStateExt;


pub trait Flowable : Pinnable {
    fn get_dependencies(&self) -> Vector<u64> {
        // For default, assume no dependencies
        Vector::new()
    }

    fn toggle_dependency(&mut self, _dependency_id: &u64) {
        // For default, don't do anything
    }
}

fn bez_to_point(rect: &Rect) -> Point {
    Point::new(rect.center().x, rect.min_y())
}

fn bez_from_point(rect: &Rect) -> Point {
    Point::new(rect.center().x, rect.max_y())
}

fn bez_from_to(from: Point, to: Point) -> CubicBez {
    let control_dist = ((to.y - from.y) / 2.0).abs();
    let from_control = from + Vec2::new(0.0, control_dist);
    let to_control = to - Vec2::new(0.0, control_dist);

    CubicBez::new(from, from_control, to_control, to)
}

fn direct_dependents<C: Data + Flowable>(root_id: u64, children: &Vector<C>) -> Vector<u64> {
    let mut result = Vector::new();
    for child in children.iter() {
        if child.get_dependencies().contains(&root_id) {
            result.push_back(child.get_id())
        }
    }
    result
}

pub struct Flow<C> {
    pub pin_board: WidgetPod<(Point, Vector<C>), PinBoard<C>>,

    linking_pin: Option<u64>,
    mouse_position: Point,
}

impl<C: Data + Debug + Flowable + PartialEq> Flow<C> {
    pub fn new<CW: Widget<C> + 'static>(
        new_widget: impl Fn() -> CW + 'static,
    ) -> Flow<C> {
        let pin_board = PinBoard::new(new_widget);
        Flow {
            pin_board: WidgetPod::new(pin_board),

            linking_pin: None,
            mouse_position: Point::ZERO,
        }
    }

    fn add_dependent_pin(&mut self, position: Point, data: &mut(Point, Vector<C>), dependency: &u64) {
        let mut pin_board = self.pin_board.widget_mut();
        let mut new_pin = pin_board.new_pin(position, data);
        new_pin.toggle_dependency(&dependency);
        let (_, child_data_vector) = data;
        child_data_vector.push_back(new_pin);
    }
}

impl<C: Data + Flowable + PartialEq + Debug> Widget<(Point, Vector<C>)> for Flow<C> {
    fn event(&mut self, ctx: &mut EventCtx, ev: &Event, data: &mut (Point, Vector<C>), env: &Env) {
        self.pin_board.event(ctx, ev, data, env);

        if ctx.is_handled() {
            return;
        }

        match ev {
            Event::MouseDown(mouse_event) => {
                if mouse_event.count == 1 && mouse_event.button.is_middle() {
                    let pin_board = self.pin_board.widget();
                    if let Some(pin_id_under_mouse) = pin_board.pin_id_under_mouse {
                        self.linking_pin = Some(pin_id_under_mouse)
                    }
                }
            },
            Event::MouseMove(mouse_event) => {
                self.mouse_position = mouse_event.pos;
                ctx.request_paint();
            },
            Event::MouseUp(mouse_event) => {
                let pin_board = self.pin_board.widget();
                if mouse_event.button.is_middle() {
                    if let Some(linking_id) = self.linking_pin {
                        if let Some(pin_id_under_mouse) = &pin_board.pin_id_under_mouse {
                            let (_, child_data_vector) = data;

                            // Find the pin under the mouse and toggle it's dependency
                            let pin_under_mouse = child_data_vector
                                .iter_mut()
                                .find(|pin| &pin.get_id() == pin_id_under_mouse);

                            if let Some(pin_under_mouse) = pin_under_mouse {
                                pin_under_mouse.toggle_dependency(&linking_id);
                                ctx.record_undo_state();
                            }
                        } else {
                            self.add_dependent_pin(mouse_event.pos, data, &linking_id);
                        }

                        self.linking_pin = None;
                    }
                } else if mouse_event.button.is_right() {
                    if let Some(pin_id_under_mouse) = &pin_board.pin_id_under_mouse {
                        let (_, child_data_vector) = data;

                        let dependent_ids = direct_dependents(*pin_id_under_mouse, child_data_vector);
                        for dependent in child_data_vector.iter_mut().filter(|child| dependent_ids.contains(&child.get_id())) {
                            dependent.toggle_dependency(&pin_id_under_mouse);
                        }

                        ctx.replace_undo_state();
                    }
                }
            },
            _ => {}
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, ev: &LifeCycle, data: &(Point, Vector<C>), env: &Env) {
        self.pin_board.lifecycle(ctx, ev, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &(Point, Vector<C>), data: &(Point, Vector<C>), env: &Env) {
        self.pin_board.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &(Point, Vector<C>), env: &Env) -> Size {
        self.pin_board.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &(Point, Vector<C>), env: &Env) {
        let pin_board = self.pin_board.widget();
        let canvas = pin_board.canvas.widget();
        for child_data in data.1.iter() {
            let child_position = canvas.get_child_position(&child_data.get_id()).expect("Could not get child position");
            let to = bez_to_point(child_position);
            for dependency_id in child_data.get_dependencies().iter() {
                let dependency_position = canvas.get_child_position(&dependency_id).expect("Could not get dependency position");
                let from = bez_from_point(dependency_position);

                let bez = bez_from_to(from, to);
                ctx.stroke(bez, &env.get(theme::BORDER_LIGHT), 2.0);
            }
        }

        if let Some(linking_id) = &self.linking_pin {
            let linking_position = canvas.get_child_position(&linking_id).expect("Could not get dependency position");
            let from = bez_from_point(linking_position);

            let to = if let Some(pin_position_under_mouse) = pin_board.pin_id_under_mouse.and_then(|id| canvas.get_child_position(&id)) {
                bez_to_point(&pin_position_under_mouse)
            } else {
                self.mouse_position
            };

            let bez = bez_from_to(from, to);
            ctx.stroke(bez, &env.get(theme::BORDER_DARK), 2.0);
        }

        self.pin_board.paint(ctx, data, env);
    }
}
