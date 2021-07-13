mod pinnable;
mod on_dependent_changed;
mod pinnable_widget_ext;

use std::any::Any;

use druid::{
    Command, Point, WidgetPod, Selector, Target, RenderContext, Vec2, theme, Rect
};
use druid::im::Vector;
use druid::kurbo::CubicBez;
use druid::widget::*;
use druid::widget::prelude::*;

pub use pinnable::Pinnable;
pub use pinnable_widget_ext::PinnableWidgetExt;
use super::canvas::Canvas;
use crate::controllers::{
    RecordUndoStateExt,
    draggable::Positioned
};

pub const DEPENDENT_STATE_CHANGED: Selector<(u64, Box<dyn Any>)> = Selector::new("PINBOARD_DEPENDENT_STATE_CHANGED");

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

fn all_dependencies<C: Data + Pinnable>(root: &C, children: &Vector<C>) -> Vector<u64> {
    let dependency_ids = root.get_dependencies();

    let mut results = dependency_ids.clone();
    for child in children.iter() {
        if dependency_ids.contains(&child.get_id()) {
            results.append(all_dependencies(child, children));
        }
    }
    results
}

fn direct_dependents<C: Data + Pinnable>(root_id: u64, children: &Vector<C>) -> Vector<u64> {
    let mut result = Vector::new();
    for child in children.iter() {
        if child.get_dependencies().contains(&root_id) {
            result.push_back(child.get_id())
        }
    }
    result
}

pub struct PinBoard<C> {
    canvas: WidgetPod<(Point, Vector<C>), Canvas<C>>,

    mouse_down_position: Option<Point>,

    linking_todo: Option<u64>,
    mouse_position: Point,
    todo_position_under_mouse: Option<Rect>,

    current_pin_id: u64,
}

impl<C: Data + Pinnable> PinBoard<C> {
    pub fn new<CW: Widget<C> + 'static>(
        new_widget: impl Fn() -> CW + 'static,
    ) -> PinBoard<C> {
        let canvas = Canvas::new(new_widget);
        PinBoard {
            canvas: WidgetPod::new(canvas),

            mouse_down_position: None,

            linking_todo: None,
            mouse_position: Point::ZERO,
            todo_position_under_mouse: None,

            current_pin_id: 0,
        }
    }

    fn new_pin(&mut self, position: Point, data: &(Point, Vector<C>)) -> C {
        let (offset, _) = data;

        let offset_position = (position.to_vec2() - offset.to_vec2()).to_point();
        let pin_id = self.current_pin_id;
        self.current_pin_id += 1;
        C::new(offset_position, pin_id)
    } 

    fn add_pin(&mut self, position: Point, data: &mut(Point, Vector<C>)) {
        let new_pin = self.new_pin(position, data);
        let (_, child_data_vector) = data;
        child_data_vector.push_back(new_pin);
    }

    fn add_dependent_pin(&mut self, position: Point, data: &mut(Point, Vector<C>), dependency: &u64) {
        let mut new_pin = self.new_pin(position, data);
        new_pin.toggle_dependency(&dependency);
        let (_, child_data_vector) = data;
        child_data_vector.push_back(new_pin);
    }
}

impl<C: Data + Positioned + Pinnable> Widget<(Point, Vector<C>)> for PinBoard<C> {
    fn event(&mut self, ctx: &mut EventCtx, ev: &Event, data: &mut (Point, Vector<C>), env: &Env) {
        self.canvas.event(ctx, ev, data, env);

        if ctx.is_handled() {
            return;
        }

        match ev {
            Event::MouseDown(mouse_event) => {
                if mouse_event.count == 1 {
                    if mouse_event.button.is_left() {
                        self.mouse_down_position = Some(mouse_event.pos);
                    } else if mouse_event.button.is_middle() {
                        if let Some(todo_position_under_mouse) = self.todo_position_under_mouse {
                            let (offset, child_data_vector) = data;
                            // Find the todo under the mouse and toggle it's dependency
                            let pin_under_mouse = child_data_vector
                                .iter_mut()
                                .find(|todo| todo.get_position() == todo_position_under_mouse.origin() - offset.to_vec2());

                            if let Some(pin_under_mouse) = pin_under_mouse {
                                self.linking_todo = Some(pin_under_mouse.get_id())
                            }
                        }
                    }
                }
            },
            Event::MouseMove(mouse_event) => {
                self.mouse_position = mouse_event.pos;
                ctx.request_paint();

                self.todo_position_under_mouse = None;
                for child_data in data.1.iter() {
                    let child_id = child_data.get_id();
                    if let Some(child_location) = self.canvas.widget().get_child_position(&child_id) {
                        if child_location.contains(mouse_event.pos) {
                            self.todo_position_under_mouse = Some(child_location.clone());
                        }
                    }
                }
            },
            Event::MouseUp(mouse_event) => {
                if let Some(mouse_down_position) = self.mouse_down_position {
                    if mouse_event.button.is_left() && mouse_event.pos == mouse_down_position {
                        self.add_pin(mouse_down_position, data);
                    } else if mouse_event.button.is_middle() {
                        if let Some(linking_id) = self.linking_todo {
                            if let Some(top_most_position) = &self.todo_position_under_mouse {
                                let (offset, child_data_vector) = data;

                                // Find the todo under the mouse and toggle it's dependency
                                let pin_under_mouse = child_data_vector
                                    .iter_mut()
                                    .find(|todo| todo.get_position() == top_most_position.origin() - offset.to_vec2());

                                if let Some(pin_under_mouse) = pin_under_mouse {
                                    pin_under_mouse.toggle_dependency(&linking_id);
                                    ctx.record_undo_state();
                                }
                            } else {
                                self.add_dependent_pin(mouse_event.pos, data, &linking_id);
                            }

                            self.linking_todo = None;
                        }
                    } else if mouse_event.button.is_right() {
                        if let Some(top_most_position) = &self.todo_position_under_mouse {
                            let (offset, child_data_vector) = data;
                            // Find the todo under the mouse and delete it
                            let pin_under_mouse = child_data_vector
                                .iter_mut()
                                .find(|todo| todo.get_position() == top_most_position.origin() - offset.to_vec2());

                            if let Some(pin_under_mouse) = pin_under_mouse {
                                let id_to_unpin = pin_under_mouse.get_id();

                                let dependent_ids = direct_dependents(id_to_unpin, child_data_vector);
                                child_data_vector.retain(|child_data| child_data.get_id() != id_to_unpin);

                                for dependent in child_data_vector.iter_mut().filter(|child| dependent_ids.contains(&child.get_id())) {
                                    dependent.toggle_dependency(&id_to_unpin);
                                }
                                ctx.record_undo_state();
                            }

                        }
                    }
                }
            },
            _ => {}
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, ev: &LifeCycle, data: &(Point, Vector<C>), env: &Env) {
        self.canvas.lifecycle(ctx, ev, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &(Point, Vector<C>), data: &(Point, Vector<C>), env: &Env) {
        self.canvas.update(ctx, data, env);

        let old_child_data_vector = &old_data.1;
        let child_data_vector = &data.1;

        for child_data in child_data_vector.iter() {
            let child_data_id = child_data.get_id();
            let old_child_data = old_child_data_vector.iter().find(|old_child_data| old_child_data.get_id() == child_data_id);

            if let Some(old_child_data) = old_child_data {
                if !old_child_data.same(child_data) {
                    for dependency in all_dependencies(child_data, child_data_vector) {
                        ctx.submit_command(Command::new(
                                DEPENDENT_STATE_CHANGED, 
                                (dependency, Box::new(child_data.clone())), 
                                Target::Auto));
                    }
                }
            }
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &(Point, Vector<C>), env: &Env) -> Size {
        self.canvas.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &(Point, Vector<C>), env: &Env) {
        let canvas = self.canvas.widget();
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

        if let Some(linking_id) = &self.linking_todo {
            let linking_position = canvas.get_child_position(&linking_id).expect("Could not get dependency position");
            let from = bez_from_point(linking_position);

            let to = if let Some(todo_position_under_mouse) = self.todo_position_under_mouse {
                bez_to_point(&todo_position_under_mouse)
            } else {
                self.mouse_position
            };

            let bez = bez_from_to(from, to);
            ctx.stroke(bez, &env.get(theme::BORDER_DARK), 2.0);
        }

        self.canvas.paint(ctx, data, env);
    }
}
