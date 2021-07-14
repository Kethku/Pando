use druid::*;
use druid::widget::Controller;

pub struct LifeCycleHandler<T> {
    callback: Box<dyn Fn(&mut EventCtx, &mut T) -> ()>,
    predicate: Box<dyn Fn(&mut LifeCycleCtx, &LifeCycle) -> bool>,
    event_raised: bool,
}

impl<T: Data> LifeCycleHandler<T> {
    pub fn new(
        callback: impl Fn(&mut EventCtx, &mut T) -> () + 'static,
        predicate: impl Fn(&mut LifeCycleCtx, &LifeCycle) -> bool + 'static,
    ) -> Self {
        Self {
            callback: Box::new(callback),
            predicate: Box::new(predicate),
            event_raised: false,
        }
    }
}

impl<T, W: Widget<T>> Controller<T, W> for LifeCycleHandler<T> {
    fn lifecycle(&mut self, child: &mut W, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        if (self.predicate)(ctx, event) {
            self.event_raised = true;
            ctx.request_anim_frame();
        }

        child.lifecycle(ctx, event, data, env);
    }

    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        if self.event_raised {
            (self.callback)(ctx, data);
            self.event_raised = false;
        } 

        child.event(ctx, event, data, env);
    }
}
