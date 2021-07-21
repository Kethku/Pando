use druid::{Point, WidgetPod};
use druid::widget::prelude::*;

use super::flow::{LinkPoint, Flowable};

pub struct LinkPoints<T> {
    inner: WidgetPod<T, Box<dyn Widget<T>>>,
    points: Vec<WidgetPod<(), Box<dyn Widget<()>>>>,
}

impl<T: Data + Flowable> Widget<T> for LinkPoints<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        for point in self.points.iter_mut() {
            point.event(ctx, event, &mut (), env);
        }

        if !ctx.is_handled() {
            self.inner.event(ctx, event, data, env);
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.inner.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {


        self.inner.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        let size = self.inner.layout(ctx, bc, data, env);
        self.inner.set_origin(ctx, data, env, Point::ORIGIN);
        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {


        self.inner.paint(ctx, data, env);
    }
}
