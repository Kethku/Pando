pub mod draggable;
pub mod take_focus;
pub mod undo_root;
pub mod handles_mouse;
pub mod event_handler;
pub mod life_cycle_handler;
pub mod selection;

use std::fmt::Debug;

use druid::{Command, Data, Event, EventCtx, LifeCycle, KbKey, Target, UpdateCtx, Widget, WidgetExt};
use druid::widget::ControllerHost;
use serde::Serialize;

use crate::AppData;
use crate::widgets::canvas::Positioned;
use draggable::DragController;
use event_handler::EventHandler;
use handles_mouse::HandlesMouse;
use life_cycle_handler::LifeCycleHandler;
use take_focus::TakeFocus;
use undo_root::{UndoRoot, RECORD_UNDO_STATE, REPLACE_UNDO_STATE};
use selection::{SelectionRoot, CLEAR_SELECTION};

pub trait DraggableWidgetExt<T, W> where T: Data + Positioned, W: Widget<T> {
    fn draggable(self, handle_mouse_events: bool) -> ControllerHost<W, DragController>;
}

impl<T: Data + Positioned, W: Widget<T> + 'static> DraggableWidgetExt<T, W> for W {
    fn draggable(self, handle_mouse_events: bool) -> ControllerHost<W, DragController> {
        self.controller(DragController::new(handle_mouse_events))
    }
}

pub trait PandoWidgetExt<T, W> where T: Data, W: Widget<T> {
    fn on_blur(self, callback: impl Fn(&mut EventCtx, &mut T) -> () + 'static) -> ControllerHost<W, LifeCycleHandler<T>>;
    fn on_enter(self, callback: impl Fn(&mut EventCtx, &mut T) -> () + 'static) -> ControllerHost<W, EventHandler<T>>;
    fn on_mouse_left(self, callback: impl Fn(&mut EventCtx, &mut T) -> () + 'static) -> ControllerHost<W, EventHandler<T>>;
    fn on_mouse_right(self, callback: impl Fn(&mut EventCtx, &mut T) -> () + 'static) -> ControllerHost<W, EventHandler<T>>;
    fn on_mouse_middle(self, callback: impl Fn(&mut EventCtx, &mut T) -> () + 'static) -> ControllerHost<W, EventHandler<T>>;
    fn on_mouse_double(self, callback: impl Fn(&mut EventCtx, &mut T) -> () + 'static) -> ControllerHost<W, EventHandler<T>>;
    fn on_mouse_shift(self, callback: impl Fn(&mut EventCtx, &mut T) -> () + 'static) -> ControllerHost<W, EventHandler<T>>;
    fn on_mouse_ctrl(self, callback: impl Fn(&mut EventCtx, &mut T) -> () + 'static) -> ControllerHost<W, EventHandler<T>>;
    fn on_mouse_alt(self, callback: impl Fn(&mut EventCtx, &mut T) -> () + 'static) -> ControllerHost<W, EventHandler<T>>;
    fn on_clear_selection(self, callback: impl Fn(&mut EventCtx, &mut T) -> () + 'static) -> ControllerHost<W, EventHandler<T>>;
    fn take_focus(self) -> ControllerHost<W, TakeFocus>;
    fn handles_mouse(self) -> ControllerHost<W, HandlesMouse>;
}

impl<T: Data + Debug + Send + Serialize, W: Widget<T> + 'static> PandoWidgetExt<T, W> for W {
    fn on_blur(self, callback: impl Fn(&mut EventCtx, &mut T) -> () + 'static) -> ControllerHost<W, LifeCycleHandler<T>> {
        self.controller(LifeCycleHandler::new(callback, |_, event| {
            if let LifeCycle::FocusChanged(false) = event {
                true
            } else {
                false
            }
        }))
    }

    fn on_enter(self, callback: impl Fn(&mut EventCtx, &mut T) -> () + 'static) -> ControllerHost<W, EventHandler<T>> {
        self.controller(EventHandler::new(callback, true, true, |_, event| {
            if let Event::KeyDown(key_event) = event {
                key_event.key == KbKey::Enter && !key_event.mods.shift()
            } else {
                false
            }
        }))
    }

    fn on_mouse_left(self, callback: impl Fn(&mut EventCtx, &mut T) -> () + 'static) -> ControllerHost<W, EventHandler<T>> {
        self.controller(EventHandler::new(callback, true, false, |_, event| {
            if let Event::MouseDown(mouse_event) = event {
                mouse_event.button.is_left() && mouse_event.count == 1
            } else {
                false
            }
        }))
    }

    fn on_mouse_right(self, callback: impl Fn(&mut EventCtx, &mut T) -> () + 'static) -> ControllerHost<W, EventHandler<T>> {
        self.controller(EventHandler::new(callback, true, false, |_, event| {
            if let Event::MouseDown(mouse_event) = event {
                mouse_event.button.is_right() && mouse_event.count == 1
            } else {
                false
            }
        }))
    }

    fn on_mouse_middle(self, callback: impl Fn(&mut EventCtx, &mut T) -> () + 'static) -> ControllerHost<W, EventHandler<T>> {
        self.controller(EventHandler::new(callback, true, false, |_, event| {
            if let Event::MouseDown(mouse_event) = event {
                mouse_event.button.is_middle() && mouse_event.count == 1
            } else {
                false
            }
        }))
    }

    fn on_mouse_double(self, callback: impl Fn(&mut EventCtx, &mut T) -> () + 'static) -> ControllerHost<W, EventHandler<T>> {
        self.controller(EventHandler::new(callback, true, false, |_, event| {
            if let Event::MouseDown(mouse_event) = event {
                mouse_event.button.is_left() && mouse_event.count == 2
            } else {
                false
            }
        }))
    }

    fn on_mouse_shift(self, callback: impl Fn(&mut EventCtx, &mut T) -> () + 'static) -> ControllerHost<W, EventHandler<T>> {
        self.controller(EventHandler::new(callback, true, false, |_, event| {
            if let Event::MouseDown(mouse_event) = event {
                mouse_event.button.is_left() && mouse_event.mods.shift() && mouse_event.count == 1
            } else {
                false
            }
        }))
    }

    fn on_mouse_ctrl(self, callback: impl Fn(&mut EventCtx, &mut T) -> () + 'static) -> ControllerHost<W, EventHandler<T>> {
        self.controller(EventHandler::new(callback, true, false, |_, event| {
            if let Event::MouseDown(mouse_event) = event {
                mouse_event.button.is_left() && mouse_event.mods.ctrl() && mouse_event.count == 1
            } else {
                false
            }
        }))
    }

    fn on_mouse_alt(self, callback: impl Fn(&mut EventCtx, &mut T) -> () + 'static) -> ControllerHost<W, EventHandler<T>> {
        self.controller(EventHandler::new(callback, true, false, |_, event| {
            if let Event::MouseDown(mouse_event) = event {
                mouse_event.button.is_left() && mouse_event.mods.alt() && mouse_event.count == 1
            } else {
                false
            }
        }))
    }

    fn on_clear_selection(self, callback: impl Fn(&mut EventCtx, &mut T) -> () + 'static) -> ControllerHost<W, EventHandler<T>> {
        self.controller(EventHandler::new(callback, false, false, |_, event| {
            if let Event::Command(command) = event {
                if command.is(CLEAR_SELECTION) {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        }))
    }

    fn take_focus(self) -> ControllerHost<W, TakeFocus> {
        self.controller(TakeFocus::new())
    }

    fn handles_mouse(self) -> ControllerHost<W, HandlesMouse> {
        self.controller(HandlesMouse { })
    }
}

pub trait RecordUndoStateExt {
    fn record_undo_state(&mut self);
    fn replace_undo_state(&mut self);
}

impl RecordUndoStateExt for EventCtx<'_, '_> {
    fn record_undo_state(&mut self) {
        self.submit_command(Command::new(RECORD_UNDO_STATE, (), Target::Auto));
    }

    fn replace_undo_state(&mut self) {
        self.submit_command(Command::new(REPLACE_UNDO_STATE, (), Target::Auto));
    }
}

impl RecordUndoStateExt for UpdateCtx<'_, '_> {
    fn record_undo_state(&mut self) {
        self.submit_command(Command::new(RECORD_UNDO_STATE, (), Target::Auto));
    }

    fn replace_undo_state(&mut self) {
        self.submit_command(Command::new(REPLACE_UNDO_STATE, (), Target::Auto));
    }
}

pub trait AppDataExt<W> where W: Widget<AppData> {
    fn undo_root(self) -> ControllerHost<W, UndoRoot>;
    fn selection_root(self) -> ControllerHost<W, SelectionRoot>;
}

impl<W: Widget<AppData> + 'static> AppDataExt<W> for W {
    fn undo_root(self) -> ControllerHost<W, UndoRoot> {
        self.controller(UndoRoot::new())
    }

    fn selection_root(self) -> ControllerHost<W, SelectionRoot> {
        self.controller(SelectionRoot { })
    }
}
