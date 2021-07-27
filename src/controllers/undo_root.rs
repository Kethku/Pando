use std::thread;
use std::sync::mpsc::{channel, Sender};

use druid::{EventCtx, Event, Env, LifeCycle, LifeCycleCtx, HotKey, Selector, SysMods, Widget};
use druid::im::Vector as ImVector;
use druid::widget::Controller;

use crate::AppData;
use crate::persistence::save;

pub const RECORD_UNDO_STATE: Selector<()> = Selector::new("RECORD_UNDO_STATE");
pub const REPLACE_UNDO_STATE: Selector<()> = Selector::new("REPLACE_UNDO_STATE");

pub struct UndoRoot {
    history: ImVector<AppData>,
    tx: Sender<AppData>
}

impl UndoRoot {
    pub fn new() -> Self {
        let (tx, rx) = channel::<AppData>();

        thread::spawn(move || {
            for data_to_save in rx.iter() {
                save(data_to_save);
            }
        });

        Self {
            history: ImVector::new(),
            tx 
        }
    }
}

impl<W: Widget<AppData>> Controller<AppData, W> for UndoRoot {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut AppData, env: &Env) {
        child.event(ctx, event, data, env);

        if ctx.is_handled() {
            return;
        }

        let undo_hotkey = HotKey::new(SysMods::Cmd, "z");

        match event {
            Event::KeyDown(key_event) => {
                if undo_hotkey.matches(key_event) {
                    if let Some(_) = self.history.pop_back() {
                        if let Some(previous_state) = self.history.last() {
                            *data = previous_state.clone();
                            ctx.request_layout();
                            ctx.request_update();
                            ctx.request_paint();
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
                    self.tx.send(data.clone()).expect("Could not send state to save system");
                } else if command.is(REPLACE_UNDO_STATE) {
                    self.history.pop_back();
                    self.history.push_back(data.clone());
                    ctx.request_layout();
                    ctx.request_update();
                    ctx.request_paint();
                    self.tx.send(data.clone()).expect("Could not send state to save system");
                }
            },
            _ => {}
        }
    }

    fn lifecycle(&mut self, child: &mut W, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &AppData, env: &Env)  {
        child.lifecycle(ctx, event, data, env);

        if let LifeCycle::WidgetAdded = event {
            self.history.push_back(data.clone())
        }
    }
}
