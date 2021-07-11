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

pub struct OnEnter<T> {
    callback: Box<dyn Fn(&mut T) -> ()>
}

impl<T: Data> OnEnter<T> {
    pub fn new(callback: impl Fn(&mut T) -> () + 'static) -> Self {
        Self {
            callback: Box::new(callback)
        }
    }
}

impl<T, W: Widget<T>> Controller<T, W> for OnEnter<T> {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        if let Event::KeyDown(key_event) = event {
            if key_event.key == KbKey::Enter && !key_event.mods.shift() {
                (self.callback)(data);
                return;
            }
        }

        child.event(ctx, event, data, env);
    }
}

pub struct OnMouseButtonDown<T> {
    button: MouseButton,
    count: u8,
    callback: Box<dyn Fn(&mut EventCtx, &mut T) -> ()>
}

impl<T: Data> OnMouseButtonDown<T> {
    pub fn new(button: MouseButton, callback: impl Fn(&mut EventCtx, &mut T) -> () + 'static) -> Self {
        Self {
            button,
            count: 1,
            callback: Box::new(callback)
        }
    }

    pub fn left(callback: impl Fn(&mut EventCtx, &mut T) -> () + 'static) -> Self {
        Self {
            button: MouseButton::Left,
            count: 1,
            callback: Box::new(callback)
        }
    }

    pub fn right(callback: impl Fn(&mut EventCtx, &mut T) -> () + 'static) -> Self {
        Self {
            button: MouseButton::Right,
            count: 1,
            callback: Box::new(callback)
        }
    }

    pub fn middle(callback: impl Fn(&mut EventCtx, &mut T) -> () + 'static) -> Self {
        Self {
            button: MouseButton::Middle,
            count: 1,
            callback: Box::new(callback)
        }
    }

    pub fn with_double_click(mut self) -> Self {
        self.count = 2;
        self
    }
}

impl<T, W: Widget<T>> Controller<T, W> for OnMouseButtonDown<T> {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        if let Event::MouseDown(mouse_event) = event {
            if mouse_event.button == self.button && mouse_event.count == self.count {
                (self.callback)(ctx, data);
                return;
            }
        }

        child.event(ctx, event, data, env);
    }
}
