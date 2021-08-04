use std::any::Any;

use druid::{Command, Point, Vec2, Selector, Target};
use druid::widget::*;
use druid::widget::prelude::*;

use super::RecordUndoStateExt;
use crate::widgets::canvas::Positioned;

pub const DRAGGING: Selector<(Box<dyn Any>, Vec2)> = Selector::new("DRAGGING");

pub struct DragController {
    child_dragged_from: Option<Point>,
    mouse_previous_position: Option<Point>,
    mouse_dragged_from: Option<Point>,
    consume_mouse_events: bool,
}

impl DragController {
    pub fn new(consume_mouse_events: bool) -> Self {
        DragController {
            child_dragged_from: None,
            mouse_previous_position: None,
            mouse_dragged_from: None,
            consume_mouse_events
        }
    }
}

impl<T: Data + Positioned, W: Widget<T>> Controller<T, W> for DragController {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        child.event(ctx, event, data, env);

        if ctx.is_handled() {
            return;
        }

        match event {
            Event::MouseDown(mouse_event) => {
                if let None = self.mouse_dragged_from {
                    if mouse_event.button.is_left() {
                        let position = data.get_position();
                        self.child_dragged_from = Some(position.clone());
                        self.mouse_previous_position = Some(mouse_event.window_pos);
                        self.mouse_dragged_from = Some(mouse_event.window_pos);
                        ctx.set_active(true);
                        ctx.request_focus();

                        if self.consume_mouse_events {
                            ctx.set_handled();
                        }
                    }
                };

            },
            Event::MouseMove(mouse_event) => {
                // Paying attention to 
                if !mouse_event.buttons.contains(druid::MouseButton::Left) {
                    if let Some(mouse_dragged_from) = self.mouse_dragged_from {
                        if mouse_dragged_from != mouse_event.pos {
                            ctx.record_undo_state();
                        }
                    }

                    ctx.set_active(false);
                    self.child_dragged_from = None;
                    self.mouse_dragged_from = None;
                }

                if let (Some(child_dragged_from), Some(mouse_dragged_from)) = (self.child_dragged_from, self.mouse_dragged_from) {
                    let current_delta = mouse_event.window_pos - mouse_dragged_from;
                    data.set_position(child_dragged_from + current_delta);
                    ctx.request_layout();
                    ctx.request_paint();

                    let mouse_delta = mouse_event.window_pos - self.mouse_previous_position.unwrap();
                    ctx.submit_command(Command::new(DRAGGING, (Box::new(data.clone()), mouse_delta), Target::Auto));
                    self.mouse_previous_position = Some(mouse_event.window_pos);

                    if self.consume_mouse_events {
                        ctx.set_handled();
                    }
                }
            },
            _ => {}
        }
    }
}
