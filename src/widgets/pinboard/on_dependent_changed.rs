use druid::{Data, Env, Event, EventCtx, Widget};
use druid::widget::Controller;

use super::{Pinnable, DEPENDENT_STATE_CHANGED};

pub struct OnDependentChanged<T> {
    callback: Box<dyn Fn(&mut EventCtx, &mut T, &T) -> ()>
}

impl<T: Data + Pinnable> OnDependentChanged<T> {
    pub fn new(callback: impl Fn(&mut EventCtx, &mut T, &T) -> () + 'static) -> Self {
        Self {
            callback: Box::new(callback)
        }
    }
}

impl<T: Pinnable + 'static, W: Widget<T>> Controller<T, W> for OnDependentChanged<T> {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        if let Event::Command(command) = event {
            if let Some((target_id, command_data)) = command.get(DEPENDENT_STATE_CHANGED) {
                if &data.get_id() == target_id {
                    if let Some(command_data) = command_data.downcast_ref::<T>() {
                        (self.callback)(ctx, data, command_data)
                    } else {
                        println!("COULD NOT CAST DEPENDENT STATE");
                    }
                }
            }
        }

        child.event(ctx, event, data, env);
    }
}
