use druid::{Data, Lens, Insets, Point, Size, Widget, WidgetExt};
use druid::im::HashSet;
use druid::theme;
use druid::widget::*;
use serde::{Serialize, Deserialize};

use super::canvas::{Positioned, Identifiable};
use super::pin_board::Pinnable;
use super::flow::{Flowable, FlowDependency, LinkPoint, Direction};
use super::link_points::LinkPointsEx;
use crate::controllers::{
    DraggableWidgetExt,
    PandoWidgetExt,
    RecordUndoStateExt,
};

#[derive(Clone, Data, Debug, PartialEq, Serialize, Deserialize)]
pub enum TodoStatus {
    Authoring,
    Waiting,
    InProgress,
    Done,
}

#[derive(Clone, Data, Debug, Lens, PartialEq, Serialize, Deserialize)]
pub struct TodoItem {
    pub id: u64,
    pub position: Point,
    pub name: String,
    pub status: TodoStatus,
    pub dependencies: HashSet<FlowDependency>,
    #[serde(default)]
    pub highlighted: bool
}

impl TodoItem {
    pub fn progress(&mut self) {
        match self.status {
            TodoStatus::Authoring => self.status = TodoStatus::Waiting,
            TodoStatus::Waiting => self.status = TodoStatus::InProgress,
            TodoStatus::InProgress => self.status = TodoStatus::Done,
            // Authoring should only happen once. So skip it and wrap to waiting
            TodoStatus::Done => self.status = TodoStatus::Waiting,
        }
    }
}

impl Positioned for TodoItem {
    fn get_position(&self) -> Point {
        self.position
    }

    fn set_position(&mut self, new_position: Point) {
        self.position = new_position;
    }
}

impl Identifiable for TodoItem {
    fn get_id(&self) -> u64 {
        self.id
    }
}

impl Pinnable for TodoItem {
    fn new(position: Point, id: u64) -> Self {
        Self {
            id,
            position,
            name: "".to_owned(),
            status: TodoStatus::Authoring,
            dependencies: HashSet::new(),
            highlighted: false,
        }
    }
}

impl Flowable for TodoItem {
    fn get_link_points(&self, size: Size) -> Vec<LinkPoint> {
        vec![
            LinkPoint {
                position: Point::new(size.width / 2.0, 0.0),
                direction: Direction::Up
            },
            LinkPoint {
                position: Point::new(size.width / 2.0, size.height),
                direction: Direction::Down
            }
        ]
    }

    fn get_dependencies(&self) -> HashSet<FlowDependency> {
        self.dependencies.clone()
    }

    fn toggle_dependency(&mut self, dependency: &FlowDependency) {
        if self.dependencies.contains(dependency) {
            self.dependencies.remove(dependency);
        } else {
            self.dependencies.insert(dependency.clone());
        }
    }

    fn break_dependencies_to(&mut self, dependency_id: u64) {
        self.dependencies.retain(|dependency| dependency.dependency_id != dependency_id);
    }
}

pub fn todo() -> impl Widget<TodoItem> {
    let contents = ViewSwitcher::<TodoItem, TodoStatus>::new(
        |todo_item, _| todo_item.status.clone(),
        |status, _, _| {
            match status {
                TodoStatus::Authoring => {
                    TextBox::multiline().with_expand().lens(TodoItem::name)
                        .on_enter(|ctx, todo| {
                            todo.progress();
                            ctx.record_undo_state();
                        })
                        .on_blur(|ctx, todo| {
                            todo.progress();
                            ctx.record_undo_state();
                        })
                        .take_focus()
                        .handles_mouse()
                        .boxed()
                },
                TodoStatus::Waiting => {
                    RawLabel::new().lens(TodoItem::name)
                        .boxed()
                },
                TodoStatus::InProgress => {
                    Flex::column()
                        .with_child(RawLabel::new().lens(TodoItem::name))
                        .with_child(Label::new("In Progress"))
                        .boxed()
                },
                TodoStatus::Done => {
                    Flex::column()
                        .with_child(RawLabel::new().with_text_color(theme::FOREGROUND_DARK).lens(TodoItem::name))
                        .with_child(Label::new("Done").with_text_color(theme::FOREGROUND_DARK))
                        .boxed()
                },
            }
        });

    contents
        .padding(Insets::uniform(10.0))
        .background(theme::BACKGROUND_LIGHT)
        .rounded(theme::BUTTON_BORDER_RADIUS)
        .border(theme::BORDER_LIGHT, theme::BUTTON_BORDER_WIDTH)
        .env_scope(|env, todo| {
            if todo.highlighted {
                env.set(theme::BORDER_LIGHT, env.get(theme::PRIMARY_LIGHT))
            }
        })
        .draggable(true)
        .on_mouse_double(|ctx, todo| {
            todo.progress();
            ctx.record_undo_state();
        })
        .on_mouse_ctrl(|ctx, todo| {
            todo.highlighted = !todo.highlighted;
            ctx.record_undo_state();
        })
        .with_link_points()
}
