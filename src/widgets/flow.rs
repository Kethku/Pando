use std::collections::HashMap;
use std::cmp::Ordering;
use std::fmt::Debug;

use druid::{
    Point, WidgetPod, Vec2, Selector
};
use druid::im::{HashSet as ImHashSet, HashMap as ImHashMap};
use druid::kurbo::CubicBez;
use druid::theme;
use druid::widget::*;
use druid::widget::prelude::*;
use serde::{Serialize, Deserialize};

use super::canvas::Canvas;
use super::pin_board::{PinBoard, Pinnable};
use super::link_points::{LinkPoints, LINK_POINT_SIZE};
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

#[derive(Clone, Data, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct FlowDependency {
    pub from_id: u64,
    pub from_link_index: usize,
    pub to_id: u64,
    pub to_link_index: usize,
}

impl FlowDependency {
    pub fn try_new(first: (u64, usize), second: (u64, usize)) -> Option<Self> {
        let (first_id, first_link_index) = first;
        let (second_id, second_link_index) = second;
        match first_id.cmp(&second_id) {
            Ordering::Less => {
                Some(FlowDependency {
                    from_id: first_id,
                    from_link_index: first_link_index,
                    to_id: second_id,
                    to_link_index: second_link_index,
                })
            },
            Ordering::Greater => {
                Some(FlowDependency {
                    from_id: second_id,
                    from_link_index: second_link_index,
                    to_id: first_id,
                    to_link_index: first_link_index,
                })
            },
            Ordering::Equal => None
        }
    }
}

pub trait Flowable : Pinnable {
    fn get_link_points(&self, size: Size) -> Vec<LinkPoint>;
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

pub struct Flow<C, W> {
    pub pin_board: WidgetPod<AppData, PinBoard<C, LinkPoints<C, W>>>,

    linking_pin: Option<(u64, usize)>,
    mouse_position: Point,

    link_points: HashMap<u64, Vec<LinkPoint>>,
}

impl<C: Data + Debug + Flowable + PartialEq, W: Widget<C>> Flow<C, W> {
    pub fn new(
        new_widget: impl Fn() -> W + 'static,
    ) -> Flow<C, W> {
        let pin_board = PinBoard::new(move || LinkPoints::new((new_widget)()));
        Flow {
            pin_board: WidgetPod::new(pin_board),

            linking_pin: None,
            mouse_position: Point::ZERO,

            link_points: HashMap::new(),
        }
    }

    pub fn canvas(&self) -> &Canvas<C, LinkPoints<C, W>> {
        self.pin_board().canvas()
    }

    pub fn canvas_mut(&mut self) -> &mut Canvas<C, LinkPoints<C, W>> {
        self.pin_board_mut().canvas_mut()
    }

    pub fn pin_board(&self) -> &PinBoard<C, LinkPoints<C, W>> {
        self.pin_board.widget()
    }

    pub fn pin_board_mut(&mut self) -> &mut PinBoard<C, LinkPoints<C, W>> {
        self.pin_board.widget_mut()
    }

    fn toggle_dependency(&mut self, data: &mut AppData, dependency: FlowDependency) {
        let (dependencies, _) = data; 
        if dependencies.contains(&dependency) {
            dependencies.remove(&dependency);
        } else {
            dependencies.insert(dependency);
        }
    }

    fn add_dependent_pin(&mut self, position: Point, data: &mut AppData, dependency_id: u64, dependency_link_index: usize) {
        let (dependencies, pin_board_data) = data;
        let pin_board = self.pin_board.widget_mut();
        let first = (dependency_id, dependency_link_index);
        let (pin_id, new_pin) = pin_board.new_pin(position, pin_board_data);
        let default_link_index = new_pin.default_link_index();
        let second = (pin_id, default_link_index);
        
        if let Some(dependency) = FlowDependency::try_new(first, second) {
            self.toggle_dependency(data, dependency);
        }
        let (_, child_data_map) = data;
    }

    fn break_dependencies_to(&mut self, data: &mut AppData, pin_id: u64) {
        let (dependencies, _) = data;
        dependencies.retain(|dependency| dependency.from_id != pin_id && dependency.to_id != pin_id);
    }
}

impl<C: Data + Flowable + PartialEq + Debug, W: Widget<C>> Widget<AppData> for Flow<C, W> {
    fn event(&mut self, ctx: &mut EventCtx, ev: &Event, data: &mut AppData, env: &Env) {
        let (flow_dependencies, pin_board_data) = data;
        self.pin_board.event(ctx, ev, pin_board_data, env);

        if ctx.is_handled() {
            return;
        }

        match ev {
            Event::Command(command) => {
                if let Some((dependency_id, link_index)) = command.get(LINK_STARTED).cloned() {
                    self.linking_pin = Some((dependency_id, link_index))
                } else if let Some(first) = command.get(LINK_FINISHED).cloned() {
                    if let Some(second) = self.linking_pin {
                        if let Some(dependency) = FlowDependency::try_new(first, second) {
                            self.toggle_dependency(data, dependency);
                        }
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
                        let (_, child_data_map) = pin_board_data;

                        for (_, child) in child_data_map.iter_mut() {
                            self.break_dependencies_to(data, *pin_id_under_mouse);
                        }

                        ctx.replace_undo_state();
                    }
                }
            },
            _ => {}
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, ev: &LifeCycle, data: &AppData, env: &Env) {
        let (_, pin_board_data) = data;
        self.pin_board.lifecycle(ctx, ev, pin_board_data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &AppData, data: &AppData, env: &Env) {
        let (_, pin_board_data) = data;
        let canvas = self.pin_board.widget().canvas.widget();

        let mut new_link_points = HashMap::new();
        let (_, child_data_map) = pin_board_data;

        for (id, child) in child_data_map {
            if let Some(child_position) = canvas.get_child_position(&id) {
                let external_child_size = child_position.size();
                let inner_child_size = Size::new(
                    external_child_size.width - LINK_POINT_SIZE,
                    external_child_size.height - LINK_POINT_SIZE);

                let external_child_offset = child_position.origin();
                let internal_child_offset = Point::new(
                    external_child_offset.x + LINK_POINT_SIZE / 2.0,
                    external_child_offset.y + LINK_POINT_SIZE / 2.0);

                let link_points: Vec<LinkPoint> = child.get_link_points(inner_child_size).into_iter()
                    .map(|link_point| link_point.with_offset(internal_child_offset))
                    .collect();
                new_link_points.insert(*id, link_points);
            }
        }

        self.link_points = new_link_points;

        self.pin_board.update(ctx, pin_board_data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &AppData, env: &Env) -> Size {
        let (_, pin_board_data) = data;
        self.pin_board.layout(ctx, bc, pin_board_data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppData, env: &Env) {
        let (dependencies, pin_board_data) = data;
        for dependency in dependencies {
            // immediately executed function expression to enable try operator
            || -> Option<()> {
                let from_link_point = self.link_points
                    .get(&dependency.from_id)?
                    .get(dependency.from_link_index)?;

                let to_link_point = self.link_points
                    .get(&dependency.to_id)?
                    .get(dependency.to_link_index)?;

                let bez = bez_from_to(
                    from_link_point.position, from_link_point.direction, 
                    to_link_point.position, to_link_point.direction.into());
                ctx.stroke(bez, &env.get(theme::BORDER_LIGHT), 2.0);

                Some(())
            }();
        }

        if let Some((linking_id, linking_point_index)) = &self.linking_pin {
            if let Some(linking_point) = self.link_points
                .get(linking_id)
                .and_then(|link_points| link_points.get(*linking_point_index)) {
                let bez = bez_from_to(linking_point.position, linking_point.direction, self.mouse_position, None);
                ctx.stroke(bez, &env.get(theme::BORDER_DARK), 2.0);
            }
        }

        self.pin_board.paint(ctx, pin_board_data, env);
    }
}
