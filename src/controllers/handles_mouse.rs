use druid::*;
use druid::widget::Controller;

pub struct HandlesMouse; 

impl<T, W: Widget<T>> Controller<T, W> for HandlesMouse {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        child.event(ctx, event, data, env);

        match event {
            Event::MouseDown(_) => ctx.set_handled(),
            Event::MouseMove(_) => ctx.set_handled(),
            Event::MouseUp(_) => ctx.set_handled(),
            _ => { }
        }
    }
}
