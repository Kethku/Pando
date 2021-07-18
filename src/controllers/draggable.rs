use druid::Point;
use druid::widget::*;
use druid::widget::prelude::*;

use crate::widgets::canvas::Positioned;

pub struct DragController {
    child_dragged_from: Option<Point>,
    mouse_dragged_from: Option<Point>,
    consume_mouse_events: bool,
}

impl DragController {
    pub fn new(consume_mouse_events: bool) -> Self {
        DragController {
            child_dragged_from: None,
            mouse_dragged_from: None,
            consume_mouse_events
        }
    }
}

impl<T: Positioned, W: Widget<T>> Controller<T, W> for DragController {
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
                        self.mouse_dragged_from = Some(mouse_event.window_pos);

                        if self.consume_mouse_events {
                            ctx.set_handled();
                        }
                    }
                };

            },
            Event::MouseMove(mouse_event) => {
                if let (Some(child_dragged_from), Some(mouse_dragged_from)) = (self.child_dragged_from, self.mouse_dragged_from) {
                    let current_delta = mouse_event.window_pos - mouse_dragged_from;
                    data.set_position(child_dragged_from + current_delta);
                    ctx.request_layout();
                    ctx.request_paint();

                    if self.consume_mouse_events {
                        ctx.set_handled();
                    }
                }
            },
            Event::MouseUp(mouse_event) => {
                if mouse_event.button.is_left() {
                    self.child_dragged_from = None;
                    self.mouse_dragged_from = None;
                }
            },
            _ => {}
        }
    }
}
