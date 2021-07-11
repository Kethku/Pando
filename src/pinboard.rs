use std::fmt::Debug;
use std::any::Any;

use druid::{
    Command, Point, WidgetPod, Selector, Target, RenderContext, Color, Vec2, theme, Rect
};
use druid::im::Vector;
use druid::kurbo::CubicBez;
use druid::widget::*;
use druid::widget::prelude::*;

use crate::canvas::Canvas;
use crate::draggable::Positioned;

pub const UNPIN: Selector<String> = Selector::new("PINBOARD_UNPIN");
pub const LINKING: Selector<String> = Selector::new("PINBOARD_LINKING");
pub const DEPENDENT_STATE_CHANGED: Selector<(String, Box<dyn Any>)> = Selector::new("PINBOARD_DEPENDENT_STATE_CHANGED");

pub trait Pinnable {
    fn get_id(&self) -> String;

    fn get_dependencies(&self) -> Vector<String> {
        // For default, assume no dependencies
        Vector::new()
    }

    fn toggle_dependency(&mut self, _dependency_id: &String) {
        // For default, don't do anything
    }
}

impl<T: Debug> Pinnable for T {
    fn get_id(&self) -> String {
        format!("{:?}", self).to_owned()
    }
}

pub struct UnpinController;

impl<T: Pinnable, W: Widget<T>> Controller<T, W> for UnpinController {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        child.event(ctx, event, data, env);

        if ctx.is_handled() {
            return;
        }

        if let Event::MouseDown(mouse_event) = event {
            if mouse_event.button.is_right() {
                ctx.submit_command(Command::new(UNPIN, data.get_id(), Target::Auto));
            }
        }
    }
}

pub struct OnDependentStateChanged<T> {
    callback: Box<dyn Fn(&mut EventCtx, &mut T, &T) -> ()>
}

impl<T: Data + Pinnable> OnDependentStateChanged<T> {
    pub fn new(callback: impl Fn(&mut EventCtx, &mut T, &T) -> () + 'static) -> Self {
        Self {
            callback: Box::new(callback)
        }
    }
}

impl<T: Pinnable + 'static, W: Widget<T>> Controller<T, W> for OnDependentStateChanged<T> {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        if let Event::Command(command) = event {
            if let Some((target_id, command_data)) = command.get(DEPENDENT_STATE_CHANGED) {
                if &data.get_id() == target_id {
                    if let Some(command_data) = command_data.downcast_ref::<T>() {
                        (self.callback)(ctx, data, command_data)
                    } else {
                        println!("COULD NOT CAST DEPENDENT STATE");
                    }
                }
            }
        }

        child.event(ctx, event, data, env);
    }
}

fn bez_points_to(rect: &Rect) -> (Point, Point) {
    let to = Point::new(rect.center().x, rect.min_y());
    let control = to + Vec2::new(0.0, -100.0);
    (to, control)
}

fn bez_points_from(rect: &Rect) -> (Point, Point) {
    let from = Point::new(rect.center().x, rect.max_y());
    let control = from + Vec2::new(0.0, 100.0);
    (from, control)
}

fn all_dependencies<C: Data + Pinnable>(root: &C, children: &Vector<C>) -> Vector<String> {
    let dependency_ids = root.get_dependencies();

    let mut results = dependency_ids.clone();
    for child in children.iter() {
        if dependency_ids.contains(&child.get_id()) {
            results.append(all_dependencies(child, children));
        }
    }
    results
}

fn direct_dependents<C: Data + Pinnable>(root_id: &String, children: &Vector<C>) -> Vector<String> {
    let mut result = Vector::new();
    for child in children.iter() {
        if child.get_dependencies().contains(root_id) {
            result.push_back(child.get_id())
        }
    }
    result
}

pub struct PinBoard<C> {
    new_pin: Box<dyn Fn(Point) -> C>,
    canvas: WidgetPod<(Point, Vector<C>), Canvas<C>>,

    mouse_down_position: Option<Point>,

    linking_todo: Option<String>,
    mouse_position: Point,
    todo_position_under_mouse: Option<Rect>,
}

impl<C: Data + Positioned + Pinnable> PinBoard<C> {
    pub fn new<CW: Widget<C> + 'static>(
        new_pin: impl Fn(Point) -> C + 'static,
        new_widget: impl Fn() -> CW + 'static,
    ) -> PinBoard<C> {
        let canvas = Canvas::new(new_widget);
        PinBoard {
            new_pin: Box::new(new_pin),
            canvas: WidgetPod::new(canvas),

            mouse_down_position: None,

            linking_todo: None,
            mouse_position: Point::ZERO,
            todo_position_under_mouse: None,
        }
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
                if mouse_event.button.is_left() && mouse_event.count == 1 {
                    self.mouse_down_position = Some(mouse_event.pos);
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
                    let (offset, child_data_vector) = data;
                    if mouse_event.button.is_left() && mouse_event.pos == mouse_down_position {
                        let offset_position = (mouse_down_position.to_vec2() - offset.to_vec2()).to_point();
                        let new_child_data = (self.new_pin)(offset_position);
                        child_data_vector.push_back(new_child_data);
                    } else if mouse_event.button.is_middle() {
                        if let Some(linking_id) = &self.linking_todo {
                            if let Some(top_most_position) = &self.todo_position_under_mouse {
                                // Find the todo under the mouse and toggle it's dependency
                                let todo_under_mouse = child_data_vector
                                    .iter_mut()
                                    .find(|todo| todo.get_position() == top_most_position.origin() - offset.to_vec2());

                                if let Some(todo_under_mouse) = todo_under_mouse {
                                    todo_under_mouse.toggle_dependency(linking_id);
                                }
                            } else {
                                // didn't drop on a todo. Create a new todo dependent on the above
                                // value.
                                let offset_position = mouse_event.pos - offset.to_vec2();
                                let mut new_pin = (self.new_pin)(offset_position);
                                new_pin.toggle_dependency(linking_id);
                                child_data_vector.push_back(new_pin);
                            }

                            self.linking_todo = None;
                        }
                    }
                }
            },
            Event::Command(command) => {
                if let Some(id_to_unpin) = command.get(UNPIN) {
                    let (_, child_data_vector) = data;
                    let dependent_ids = direct_dependents(id_to_unpin, child_data_vector);
                    child_data_vector.retain(|child_data| child_data.get_id() != *id_to_unpin);

                    for dependent in child_data_vector.iter_mut().filter(|child| dependent_ids.contains(&child.get_id())) {
                        dependent.toggle_dependency(id_to_unpin);
                    }
                } else if let Some(id_to_link) = command.get(LINKING) {
                    self.linking_todo = Some(id_to_link.clone());
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
        self.canvas.paint(ctx, data, env);
        let canvas = self.canvas.widget();

        for child_data in data.1.iter() {
            let child_position = canvas.get_child_position(&child_data.get_id()).expect("Could not get child position");
            let (to, control_2) = bez_points_to(child_position);
            for dependency_id in child_data.get_dependencies().iter() {
                let dependency_position = canvas.get_child_position(&dependency_id).expect("Could not get dependency position");
                let (from, control_1) = bez_points_from(dependency_position);

                let path = CubicBez::new(from, control_1, control_2, to);
                ctx.stroke(path, &env.get(theme::BORDER_LIGHT), 2.0);
            }
        }

        if let Some(linking_id) = &self.linking_todo {
            let linking_position = canvas.get_child_position(&linking_id).expect("Could not get dependency position");
            let (from, control_1) = bez_points_from(linking_position);

            let (to, control_2) = if let Some(todo_position_under_mouse) = self.todo_position_under_mouse {
                bez_points_to(&todo_position_under_mouse)
            } else {
                let to = self.mouse_position;
                let control_2 = to + Vec2::new(0.0, -100.0);
                (to, control_2)
            };

            let path = CubicBez::new(from, control_1, control_2, to);
            ctx.stroke(path, &env.get(theme::BORDER_DARK), 2.0);
        }
    }
}
