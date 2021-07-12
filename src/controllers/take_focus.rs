use druid::*;
use druid::widget::Controller;

pub struct TakeFocus { 
    grab_focus: bool 
}

impl TakeFocus {
    pub fn new() -> Self {
        Self {
            grab_focus: false
        }
    }
}

impl<T, W: Widget<T>> Controller<T, W> for TakeFocus {
    fn lifecycle(&mut self, child: &mut W, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        if let LifeCycle::WidgetAdded = event {
            self.grab_focus = true;
            ctx.request_anim_frame();
        }
        child.lifecycle(ctx, event, data, env);
    }

    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        if self.grab_focus {
            self.grab_focus = false;
            ctx.request_focus();
        }

        child.event(ctx, event, data, env);
    }
}
