use druid::{Color, WidgetPod};
use druid::widget::prelude::*;

use super::flow::{LinkPoint, Flowable};

const LINK_POINT_SIZE: f64 = 5.0;

impl Widget<()> for LinkPoint {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut (), _env: &Env) {
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &(), _env: &Env) {
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &(), _data: &(), _env: &Env) {
    }

    fn layout(&mut self, _ctx: &mut LayoutCtx, _bc: &BoxConstraints, _data: &(), _env: &Env) -> Size {
        Size::new(LINK_POINT_SIZE, LINK_POINT_SIZE)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &(), _env: &Env) {
        let rect = druid::kurbo::Rect::new(0.0, 0.0, LINK_POINT_SIZE, LINK_POINT_SIZE);
        ctx.fill(rect, &Color::RED);
    }
}

pub struct LinkPoints<T> {
    inner: WidgetPod<T, Box<dyn Widget<T>>>,
    points: Vec<WidgetPod<(), LinkPoint>>,
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
        let inner_size = self.inner.layout(ctx, bc, data, env);
        self.points = data.get_link_points(inner_size).into_iter().map(|link_point| WidgetPod::new(link_point)).collect();
        inner_size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        for point in self.points.iter_mut() {
            point.paint(ctx, &(), env);
        }

        self.inner.paint(ctx, data, env);
    }
}

pub trait LinkPointsEx<T> {
    fn with_link_points(self) -> LinkPoints<T>;
}
