use druid::{Data, Lens, Insets, Point, Widget, WidgetExt};
use druid::im::Vector;
use druid::theme;
use druid::widget::*;
use serde::{Serialize, Deserialize};

use super::pinboard::Pinnable;
use crate::controllers::{
    DraggableWidgetExt,
    PandoWidgetExt,
    RecordUndoStateExt,
    draggable::Positioned
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
    id: u64,
    position: Point,
    name: String,
    status: TodoStatus,
    dependencies: Vector<u64>,
    #[serde(default)]
    highlighted: bool
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

impl Pinnable for TodoItem {
    fn new(position: Point, id: u64) -> Self {
        Self {
            id,
            position,
            name: "".to_owned(),
            status: TodoStatus::Authoring,
            dependencies: Vector::new(),
            highlighted: false
        }
    }

    fn get_id(&self) -> u64 {
        self.id
    }

    fn get_dependencies(&self) -> Vector<u64> {
        self.dependencies.clone()
    }

    fn toggle_dependency(&mut self, dependency_id: &u64) {
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
}
