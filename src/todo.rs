use druid::{Command, Data, Lens, Point, Insets, Target, Widget, WidgetExt};
use druid::im::Vector;
use druid::theme;
use druid::widget::*;

use crate::draggable::{DragController, Positioned};
use crate::pinboard::{Pinnable, UnpinController, OnDependentStateChanged, LINKING};
use crate::utils::{TakeFocus, OnEnter, OnMouseButtonDown};

#[derive(Clone, Data, PartialEq)]
pub enum TodoStatus {
    Authoring,
    Waiting,
    InProgress,
    Done,
}

#[derive(Clone, Data, Lens, PartialEq)]
pub struct TodoItem {
    position: Point,
    name: String,
    status: TodoStatus,
    dependencies: Vector<String>,
}

impl TodoItem {
    pub fn new(position: Point) -> Self {
        Self {
            position,
            name: "".to_owned(),
            status: TodoStatus::Authoring,
            dependencies: Vector::new(),
        }
    }

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

impl Pinnable for TodoItem {
    fn get_id(&self) -> String {
        self.name.clone()
    }

    fn get_dependencies(&self) -> Vector<String> {
        self.dependencies.clone()
    }

    fn toggle_dependency(&mut self, dependency_id: &String) {
        if &self.get_id() == dependency_id {
            // Set dependency on self, swap to editing mode
            self.status = TodoStatus::Authoring;
        } else if self.dependencies.contains(dependency_id) {
            self.dependencies.retain(|id| id != dependency_id);
        } else {
            self.dependencies.push_back(dependency_id.clone());
        }
    }
}

pub fn todo() -> impl Widget<TodoItem> {
    let contents = ViewSwitcher::<TodoItem, TodoStatus>::new(
        |todo_item, _| todo_item.status.clone(),
        |status, _, _| {
            match status {
                TodoStatus::Authoring => {
                    TextBox::multiline().lens(TodoItem::name)
                        .controller(OnEnter::<TodoItem>::new(|todo| todo.progress()))
                        .controller(TakeFocus::new())
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
        .controller(DragController::new(true))
        .controller(UnpinController {})
        .controller(OnMouseButtonDown::<TodoItem>::left(
                |_, todo| todo.progress()).with_double_click())
        .controller(OnMouseButtonDown::<TodoItem>::middle(
                |ctx, todo| ctx.submit_command(Command::new(LINKING, todo.get_id(), Target::Auto))))
        .controller(OnDependentStateChanged::<TodoItem>::new(
                |_, todo, changed_todo| {
                    match changed_todo.status {
                        TodoStatus::Done => todo.status = TodoStatus::Done,
                        _ => {}
                    }
                }))
}
