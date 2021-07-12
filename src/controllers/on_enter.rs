use druid::*;
use druid::widget::Controller;

pub struct OnEnter<T> {
    callback: Box<dyn Fn(&mut EventCtx, &mut T) -> ()>
}

impl<T: Data> OnEnter<T> {
    pub fn new(callback: impl Fn(&mut EventCtx, &mut T) -> () + 'static) -> Self {
        Self {
            callback: Box::new(callback)
        }
    }
}

impl<T, W: Widget<T>> Controller<T, W> for OnEnter<T> {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        if let Event::KeyDown(key_event) = event {
            if key_event.key == KbKey::Enter && !key_event.mods.shift() {
                (self.callback)(ctx, data);
                return;
            }
        }

        child.event(ctx, event, data, env);
    }
}
