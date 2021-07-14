pub mod draggable;
pub mod take_focus;
pub mod undo_root;
pub mod handles_mouse;
pub mod event_handler;
pub mod life_cycle_handler;

use std::fmt::Debug;

use druid::{Command, Data, Event, EventCtx, LifeCycle, KbKey, Target, UpdateCtx, Widget, WidgetExt};
use druid::widget::ControllerHost;
use serde::Serialize;

use draggable::{DragController, Positioned};
use event_handler::EventHandler;
use handles_mouse::HandlesMouse;
use life_cycle_handler::LifeCycleHandler;
use take_focus::TakeFocus;
use undo_root::{UndoRoot, RECORD_UNDO_STATE};

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
    fn take_focus(self) -> ControllerHost<W, TakeFocus>;
    fn undo_root(self) -> ControllerHost<W, UndoRoot<T>>;
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
                dbg!(mouse_event.button.is_left() && mouse_event.count == 1)
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

    fn take_focus(self) -> ControllerHost<W, TakeFocus> {
        self.controller(TakeFocus::new())
    }

    fn undo_root(self) -> ControllerHost<W, UndoRoot<T>> {
        self.controller(UndoRoot::new())
    }

    fn handles_mouse(self) -> ControllerHost<W, HandlesMouse> {
        self.controller(HandlesMouse { })
    }
}

pub trait RecordUndoStateExt {
    fn record_undo_state(&mut self);
}

impl RecordUndoStateExt for EventCtx<'_, '_> {
    fn record_undo_state(&mut self) {
        self.submit_command(Command::new(RECORD_UNDO_STATE, (), Target::Auto));
    }
}

impl RecordUndoStateExt for UpdateCtx<'_, '_> {
    fn record_undo_state(&mut self) {
        self.submit_command(Command::new(RECORD_UNDO_STATE, (), Target::Auto));
    }
}
