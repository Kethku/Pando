pub mod draggable;
pub mod take_focus;
pub mod on_enter;
pub mod on_mouse_button_down;
pub mod undo_root;
pub mod handles_mouse;

use std::fmt::Debug;

use druid::{Command, Data, EventCtx, Target, UpdateCtx, Widget, WidgetExt};
use druid::widget::ControllerHost;
use serde::Serialize;

use draggable::{DragController, Positioned};
use take_focus::TakeFocus;
use on_enter::OnEnter;
use on_mouse_button_down::OnMouseButtonDown;
use undo_root::{UndoRoot, RECORD_UNDO_STATE};
use handles_mouse::HandlesMouse;

pub trait DraggableWidgetExt<T, W> where T: Data + Positioned, W: Widget<T> {
    fn draggable(self, handle_mouse_events: bool) -> ControllerHost<W, DragController>;
}

impl<T: Data + Positioned, W: Widget<T> + 'static> DraggableWidgetExt<T, W> for W {
    fn draggable(self, handle_mouse_events: bool) -> ControllerHost<W, DragController> {
        self.controller(DragController::new(handle_mouse_events))
    }
}

pub trait PandoWidgetExt<T, W> where T: Data, W: Widget<T> {
    fn take_focus(self) -> ControllerHost<W, TakeFocus>;
    fn on_enter(self, callback: impl Fn(&mut EventCtx, &mut T) -> () + 'static) -> ControllerHost<W, OnEnter<T>>;
    fn on_mouse_left(self, callback: impl Fn(&mut EventCtx, &mut T) -> () + 'static) -> ControllerHost<W, OnMouseButtonDown<T>>;
    fn on_mouse_right(self, callback: impl Fn(&mut EventCtx, &mut T) -> () + 'static) -> ControllerHost<W, OnMouseButtonDown<T>>;
    fn on_mouse_middle(self, callback: impl Fn(&mut EventCtx, &mut T) -> () + 'static) -> ControllerHost<W, OnMouseButtonDown<T>>;
    fn on_mouse_double(self, callback: impl Fn(&mut EventCtx, &mut T) -> () + 'static) -> ControllerHost<W, OnMouseButtonDown<T>>;
    fn undo_root(self) -> ControllerHost<W, UndoRoot<T>>;
    fn handles_mouse(self) -> ControllerHost<W, HandlesMouse>;
}

impl<T: Data + Debug + Send + Serialize, W: Widget<T> + 'static> PandoWidgetExt<T, W> for W {
    fn take_focus(self) -> ControllerHost<W, TakeFocus> {
        self.controller(TakeFocus::new())
    }

    fn on_enter(self, callback: impl Fn(&mut EventCtx, &mut T) -> () + 'static) -> ControllerHost<W, OnEnter<T>> {
        self.controller(OnEnter::new(callback))
    }

    fn on_mouse_left(self, callback: impl Fn(&mut EventCtx, &mut T) -> () + 'static) -> ControllerHost<W, OnMouseButtonDown<T>> {
        self.controller(OnMouseButtonDown::left(callback))
    }

    fn on_mouse_right(self, callback: impl Fn(&mut EventCtx, &mut T) -> () + 'static) -> ControllerHost<W, OnMouseButtonDown<T>> {
        self.controller(OnMouseButtonDown::right(callback))
    }

    fn on_mouse_middle(self, callback: impl Fn(&mut EventCtx, &mut T) -> () + 'static) -> ControllerHost<W, OnMouseButtonDown<T>> {
        self.controller(OnMouseButtonDown::middle(callback))
    }

    fn on_mouse_double(self, callback: impl Fn(&mut EventCtx, &mut T) -> () + 'static) -> ControllerHost<W, OnMouseButtonDown<T>> {
        self.controller(OnMouseButtonDown::left(callback).with_double_click())
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
