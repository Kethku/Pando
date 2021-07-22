use std::collections::HashMap;
use std::fmt::Debug;

use druid::{
    Point, WidgetPod, Vec2, Selector
};
use druid::im::{HashSet, Vector};
use druid::kurbo::CubicBez;
use druid::theme;
use druid::widget::*;
use druid::widget::prelude::*;
use serde::{Serialize, Deserialize};

use super::pin_board::{PinBoard, Pinnable};
use crate::controllers::RecordUndoStateExt;

pub const LINK_STARTED: Selector<(u64, usize)> = Selector::new("LINK_STARTED");
pub const LINK_FINISHED: Selector<(u64, usize)> = Selector::new("LINK_FINISHED");
pub const LINK_STOPPED: Selector<()> = Selector::new("LINK_STOPPED");

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Up, Down, Left, Right
}

impl Direction {
    fn to_unit_vec(&self) -> Vec2 {
        match self {
            Direction::Up => Vec2::new(0.0, -1.0),
            Direction::Down => Vec2::new(0.0, 1.0),
            Direction::Left => Vec2::new(-1.0, 0.0),
            Direction::Right => Vec2::new(1.0, 0.0),
        }
    }

    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    fn distance_along(&self, from: Point, to: Point) -> f64 {
        match self {
            Direction::Up | Direction::Down => (to.y - from.y).abs(),
            Direction::Left | Direction::Right => (to.x - from.x).abs(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct LinkPoint {
    pub position: Point,
    pub direction: Direction
}

impl LinkPoint {
    pub fn with_offset(&self, offset: Point) -> LinkPoint {
        LinkPoint {
            position: self.position + offset.to_vec2(),
            direction: self.direction
        }
    }
}

#[derive(Clone, Data, Debug, Eq, Hash,  PartialEq, Serialize, Deserialize)]
pub struct FlowDependency {
    pub local_link_index: usize,
    pub dependency_id: u64,
    pub dependency_link_index: usize,
}

pub trait Flowable : Pinnable {
    fn get_link_points(&self, size: Size) -> Vec<LinkPoint>;

    fn get_dependencies(&self) -> HashSet<FlowDependency>;
    fn toggle_dependency(&mut self, dependency: &FlowDependency);
    fn break_dependencies_to(&mut self, dependency_id: u64);

    fn default_link_index(&self) -> usize {
        // Default to the first dependency
        0
    }
}

fn bez_from_to(from: Point, from_dir: Direction, to: Point, to_dir: Option<Direction>) -> CubicBez {
    let to_dir = to_dir.unwrap_or_else(|| from_dir.opposite());

    let from_control = from + from_dir.to_unit_vec() * from_dir.distance_along(from, to) / 2.0;
    let to_control = to + to_dir.to_unit_vec() * to_dir.distance_along(from, to) / 2.0;

    CubicBez::new(from, from_control, to_control, to)
}

pub struct Flow<C> {
    pub pin_board: WidgetPod<(Point, Vector<C>), PinBoard<C>>,

    linking_pin: Option<(u64, usize)>,
    mouse_position: Point,

    link_points: HashMap<u64, Vec<LinkPoint>>,
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

            link_points: HashMap::new(),
        }
    }

    fn add_dependent_pin(&mut self, position: Point, data: &mut(Point, Vector<C>), dependency_id: u64, dependency_link_index: usize) {
        let pin_board = self.pin_board.widget_mut();
        let mut new_pin = pin_board.new_pin(position, data);
        let default_link_index = new_pin.default_link_index();
        let dependency = FlowDependency {
            local_link_index: default_link_index,
            dependency_id,
            dependency_link_index,
        };
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
            Event::Command(command) => {
                if let Some((dependency_id, link_index)) = command.get(LINK_STARTED).cloned() {
                    self.linking_pin = Some((dependency_id, link_index))
                } else if let Some((dependency_id, dependency_point_index)) = command.get(LINK_FINISHED).cloned() {
                    if let Some((linking_id, linking_point_index)) = self.linking_pin {
                        let (_, child_data) = data;
                        let linking_pin = child_data.iter_mut()
                            .find(|pin| pin.get_id() == linking_id)
                            .expect("Could not find linking pin");
                        linking_pin.toggle_dependency(&FlowDependency {
                            local_link_index: linking_point_index,
                            dependency_id,
                            dependency_link_index: dependency_point_index
                        });
                    }

                    self.linking_pin = None;
                } else if command.is(LINK_STOPPED) {
                    self.linking_pin = None;
                }
            },
            Event::MouseMove(mouse_event) => {
                self.mouse_position = mouse_event.pos;
                ctx.request_paint();
            },
            Event::MouseUp(mouse_event) => {
                let pin_board = self.pin_board.widget();
                if mouse_event.button.is_left() {
                    if let Some((linking_id, link_point_index)) = self.linking_pin {
                        self.add_dependent_pin(mouse_event.pos, data, linking_id, link_point_index);

                        self.linking_pin = None;
                    }
                } else if mouse_event.button.is_right() {
                    if let Some(pin_id_under_mouse) = &pin_board.pin_id_under_mouse {
                        let (_, child_data_vector) = data;

                        for child in child_data_vector.iter_mut() {
                            child.break_dependencies_to(*pin_id_under_mouse);
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
        let canvas = self.pin_board.widget().canvas.widget();

        let mut new_link_points = HashMap::new();
        let (_, child_data_vector) = data;

        for child in child_data_vector {
            let id = child.get_id();
            if let Some(child_position) = canvas.get_child_position(&id) {
                let link_points: Vec<LinkPoint> = child.get_link_points(child_position.size()).into_iter()
                    .map(|link_point| link_point.with_offset(child_position.origin()))
                    .collect();
                new_link_points.insert(id, link_points);
            }
        }

        self.link_points = new_link_points;

        self.pin_board.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &(Point, Vector<C>), env: &Env) -> Size {
        self.pin_board.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &(Point, Vector<C>), env: &Env) {
        for child_data in data.1.iter() {
            if let Some(child_link_points) = self.link_points.get(&child_data.get_id()) {
                for dependency in child_data.get_dependencies().iter() {
                    let child_link_point = child_link_points.get(dependency.local_link_index).expect("Could not get child link point");
                    if let Some(dependency_link_points) = self.link_points.get(&dependency.dependency_id) {
                        let dependency_link_point = dependency_link_points.get(dependency.dependency_link_index).expect("Could not get dependency link point");

                        let bez = bez_from_to(
                            child_link_point.position, child_link_point.direction, 
                            dependency_link_point.position, dependency_link_point.direction.into());
                        ctx.stroke(bez, &env.get(theme::BORDER_LIGHT), 2.0);
                    }
                }
            }
        }

        if let Some((linking_id, linking_point_index)) = &self.linking_pin {
            let linking_point = self.link_points
                .get(linking_id).expect("Could not get linking points")
                .get(*linking_point_index).expect("Could not get linking point");

            let bez = bez_from_to(linking_point.position, linking_point.direction, self.mouse_position, None);
            ctx.stroke(bez, &env.get(theme::BORDER_DARK), 2.0);
        }

        self.pin_board.paint(ctx, data, env);
    }
}
