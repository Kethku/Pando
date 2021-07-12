use druid::*;
use druid::widget::Controller;

pub struct OnMouseButtonDown<T> {
    button: MouseButton,
    count: u8,
    callback: Box<dyn Fn(&mut EventCtx, &mut T) -> ()>
}

impl<T: Data> OnMouseButtonDown<T> {
    pub fn new(button: MouseButton, count: u8, callback: impl Fn(&mut EventCtx, &mut T) -> () + 'static) -> Self {
        Self {
            button,
            count,
            callback: Box::new(callback)
        }
    }

    pub fn left(callback: impl Fn(&mut EventCtx, &mut T) -> () + 'static) -> Self {
        Self::new(MouseButton::Left, 1, callback)
    }

    pub fn right(callback: impl Fn(&mut EventCtx, &mut T) -> () + 'static) -> Self {
        Self::new(MouseButton::Right, 1, callback)
    }

    pub fn middle(callback: impl Fn(&mut EventCtx, &mut T) -> () + 'static) -> Self {
        Self::new(MouseButton::Middle, 1, callback)
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
