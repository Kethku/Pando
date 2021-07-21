use std::collections::HashMap;
use std::fmt::Debug;

use druid::{
    Point, WidgetPod, RenderContext, Vec2, Selector
};
use druid::im::{HashSet, Vector};
use druid::kurbo::CubicBez;
use druid::widget::*;
use druid::widget::prelude::*;
use serde::{Serialize, Deserialize};

use super::pin_board::{PinBoard, Pinnable};
use crate::controllers::RecordUndoStateExt;

pub const START_LINK: Selector<(u64, usize)> = Selector::new("START_LINK");

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
    fn with_offset(&self, offset: Point) -> LinkPoint {
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
    let to_control = to - to_dir.to_unit_vec() * to_dir.distance_along(from, to) / 2.0;

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

    fn closest_link_point(&self, flowable: &C, position: Point) -> Option<(usize, LinkPoint)> {
        let mut closest = None;
        let mut closest_distance = 0.0;

        if let Some(link_points) = self.link_points.get(&flowable.get_id()) {
            for (index, link_point) in link_points.iter().enumerate() {
                let distance = link_point.position.distance(position);
                if closest.is_none() || closest_distance > distance {
                    closest = Some((index, link_point.clone()));
                    closest_distance = distance;
                }
            }
        }

        closest
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
                if let Some((dependency_id, link_index)) = command.get(START_LINK).cloned() {
                    self.linking_pin = Some((dependency_id, link_index))
                }
            },
            Event::MouseMove(mouse_event) => {
                self.mouse_position = mouse_event.pos;
                ctx.request_paint();
            },
            Event::MouseUp(mouse_event) => {
                let pin_board = self.pin_board.widget();
                if mouse_event.button.is_middle() {
                    if let Some((linking_id, link_point_index)) = self.linking_pin {
                        if let Some(pin_id_under_mouse) = &pin_board.pin_id_under_mouse {
                            let (_, child_data_vector) = data;

                            // Find the pin under the mouse and toggle it's dependency
                            let pin_under_mouse = child_data_vector
                                .iter_mut()
                                .find(|pin| &pin.get_id() == pin_id_under_mouse);


                            if let Some(pin_under_mouse) = pin_under_mouse {
                                if let Some((link_index, _)) = self.closest_link_point(pin_under_mouse, mouse_event.pos) {
                                    let flow_dependency = FlowDependency {
                                        local_link_index: link_point_index,
                                        dependency_id: *pin_id_under_mouse,
                                        dependency_link_index: link_index
                                    };
                                    pin_under_mouse.toggle_dependency(&flow_dependency);
                                    ctx.record_undo_state();
                                }
                            }
                        } else {
                            self.add_dependent_pin(mouse_event.pos, data, linking_id, link_point_index);
                        }

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
        // let pin_board = self.pin_board.widget();
        // for child_data in data.1.iter() {
            // let child_link_points = child_data.get_link_points();
            // for dependency in child_data.get_dependencies().iter() {
            //     let dependency_position = canvas.get_child_position(&dependency.dependency_id).expect("Could not get dependency position");

            //     let bez = bez_from_to(from, to);
            //     ctx.stroke(bez, &env.get(theme::BORDER_LIGHT), 2.0);
            // }
        // }

        // if let Some(linking_id) = &self.linking_pin {
        //     let from = bez_from_point(linking_position);

        //     let to = if let Some(pin_position_under_mouse) = pin_board.pin_id_under_mouse.and_then(|id| canvas.get_child_position(&id)) {
        //         bez_to_point(&pin_position_under_mouse)
        //     } else {
        //         self.mouse_position
        //     };

        //     let bez = bez_from_to(from, to);
        //     ctx.stroke(bez, &env.get(theme::BORDER_DARK), 2.0);
        // }

        self.pin_board.paint(ctx, data, env);

        for link_point in self.link_points.values().flatten() {
            dbg!(link_point.position);
            let circle = druid::kurbo::Circle::new(link_point.position, 5.0);
            ctx.fill(circle, &druid::Color::RED);
        }
    }
}
