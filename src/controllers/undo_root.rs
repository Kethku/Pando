use std::fmt::Debug;
use std::thread;
use std::sync::mpsc::channel;

use druid::{Data, EventCtx, Event, Env, LifeCycle, LifeCycleCtx, HotKey, Selector, SysMods, Widget};
use druid::im::Vector;
use druid::widget::Controller;
use serde::Serialize;

use crate::save::save;

pub const RECORD_UNDO_STATE: Selector<()> = Selector::new("RECORD_UNDO_STATE");

pub struct UndoRoot<T> {
    history: Vector<T>,
}

impl<T: Data + Send + Serialize> UndoRoot<T> {
    pub fn new() -> Self {
        let (tx, rx) = channel::<T>();

        thread::spawn(move || {
            for data_to_save in rx.iter() {
                save(data_to_save);
            }
        });

        Self {
            history: Vector::new()
        }
    }
}

impl<T: Data + Debug, W: Widget<T>> Controller<T, W> for UndoRoot<T> {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        child.event(ctx, event, data, env);

        if ctx.is_handled() {
            return;
        }

        let undo_hotkey = HotKey::new(SysMods::None, "u");

        match event {
            Event::KeyDown(key_event) => {
                if undo_hotkey.matches(key_event) {
                    if let Some(_) = self.history.pop_back() {
                        if let Some(previous_state) = self.history.last() {
                            *data = previous_state.clone();
                        }
                    }
                }
            },
            Event::Command(command) => {
                if command.is(RECORD_UNDO_STATE) {
                    self.history.push_back(data.clone());
                    ctx.request_layout();
                    ctx.request_update();
                    ctx.request_paint();
                }
            },
            _ => {}
        }
    }

    fn lifecycle(&mut self, child: &mut W, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env)  {
        child.lifecycle(ctx, event, data, env);

        if let LifeCycle::WidgetAdded = event {
            self.history.push_back(data.clone())
        }
    }
}
