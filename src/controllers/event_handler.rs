use druid::*;
use druid::widget::Controller;

pub struct EventHandler<T> {
    callback: Box<dyn Fn(&mut EventCtx, &mut T) -> ()>,
    handle_event: bool,
    consume_event: bool,
    predicate: Box<dyn Fn(&mut EventCtx, &Event) -> bool>,
}

impl<T: Data> EventHandler<T> {
    pub fn new(
        callback: impl Fn(&mut EventCtx, &mut T) -> () + 'static,
        handle_event: bool,
        consume_event: bool,
        predicate: impl Fn(&mut EventCtx, &Event) -> bool + 'static,
    ) -> Self {
        Self {
            callback: Box::new(callback),
            handle_event,
            consume_event,
            predicate: Box::new(predicate),
        }
    }
}

impl<T, W: Widget<T>> Controller<T, W> for EventHandler<T> {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        if (self.predicate)(ctx, event) {
            (self.callback)(ctx, data);
            if self.handle_event {
                ctx.set_handled();
            }

            if self.consume_event {
                return;
            }
        }

        child.event(ctx, event, data, env);
    }
}
