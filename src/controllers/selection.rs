use std::collections::HashSet;
use std::iter::FromIterator;

use clipboard::{ClipboardContext, ClipboardProvider};
use druid::{Command, EventCtx, Event, Env, HotKey, Selector, SysMods, Target, Widget};
use druid::widget::Controller;
use serde::{Serialize, Deserialize};

use crate::{
    AppData,
    controllers::RecordUndoStateExt,
    widgets::{
        flow::FlowDependency,
        todo::TodoItem
    }
};

pub const CLEAR_SELECTION: Selector<()> = Selector::new("CLEAR_SELECTION");

pub struct SelectionRoot {}

#[derive(Serialize, Deserialize)]
struct ClipboardTodos {
    todos: Vec<TodoItem>,
    dependencies: Vec<FlowDependency>,
}

impl ClipboardTodos {
    // Sneaky hack. Just add the current max to the ids of in the list. This will result in a gap,
    // but will ensure no collisions
    fn adjust_ids(&mut self, current_max_id: u64) {
        for todo in self.todos.iter_mut() {
            todo.id += current_max_id;
        }

        for dependency in self.dependencies.iter_mut() {
            dependency.from_id += current_max_id;
            dependency.to_id += current_max_id;
        }
    }

    fn copy_selected(data: &AppData) {
        let todos: Vec<TodoItem> = data.todos.values()
            .cloned()
            .filter(|todo| todo.selected)
            .collect();
        let selected_ids: HashSet<u64> = HashSet::from_iter(todos.iter().map(|todo| todo.id));
        let dependencies = data.dependencies.iter()
            .cloned()
            .filter(|dependency| selected_ids.contains(&dependency.from_id) && selected_ids.contains(&dependency.to_id))
            .collect();

        let clipboard_todos = ClipboardTodos {
            todos,
            dependencies
        };
        let clipboard_todos_json = serde_json::to_string(&clipboard_todos).expect("Could not serialize selected todos to json");
        let mut clipboard_ctx = ClipboardContext::new().expect("Could not construct clipboard context");
        clipboard_ctx.set_contents(clipboard_todos_json).expect("Could not set selected todo json to clipboard");
    }

    fn paste_todos(data: &mut AppData, ctx: &mut EventCtx) {
        let mut clipboard_ctx = ClipboardContext::new().expect("Could not construct clipboard context");
        let clipboard_todos_json = clipboard_ctx.get_contents().expect("Could not get todo json from clipboard");
        if let Ok(mut clipboard_todos) = serde_json::from_str::<ClipboardTodos>(&clipboard_todos_json) {
            let current_max_id = data.todos.keys().copied().max().unwrap_or(0);
            clipboard_todos.adjust_ids(current_max_id);

            for todo in clipboard_todos.todos {
                data.todos.insert(todo.id, todo.clone());
            }
            for dependency in clipboard_todos.dependencies {
                data.dependencies.insert(dependency.clone());
            }

            ctx.record_undo_state();
        } // Ignore deserialize error and assume it wasn't valid todo data.
    }
}

impl<W: Widget<AppData>> Controller<AppData, W> for SelectionRoot {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut AppData, env: &Env) {
        if let Event::MouseDown(mouse_event) = event {
            if mouse_event.button.is_left() && !mouse_event.mods.shift() {
                ctx.submit_command(Command::new(CLEAR_SELECTION, (), Target::Auto));
            }
        }

        child.event(ctx, event, data, env);

        if ctx.is_handled() {
            return;
        }

        let copy_hotkey = HotKey::new(SysMods::Cmd, "c");
        let paste_hotkey = HotKey::new(SysMods::Cmd, "v");

        if let Event::KeyDown(key_event) = event {

            if copy_hotkey.matches(key_event) {
                ClipboardTodos::copy_selected(data);
            } else if paste_hotkey.matches(key_event) {
                ClipboardTodos::paste_todos(data, ctx);
            }
        }
    }
}
