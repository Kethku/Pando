use druid::{Command, Point, Target, WidgetPod};
use druid::theme;
use druid::kurbo::Rect;
use druid::widget::prelude::*;

use super::flow::{LinkPoint, Flowable, LINK_STARTED, LINK_FINISHED, LINK_STOPPED};

pub const LINK_POINT_SIZE: f64 = 10.0;

fn link_point_rect(point: &LinkPoint) -> Rect {
    Rect::from_center_size(point.position, Size::new(LINK_POINT_SIZE, LINK_POINT_SIZE))
}

pub struct LinkPoints<T, W> {
    inner: WidgetPod<T, W>,
    points: Vec<LinkPoint>,

    mouse_position: Option<Point>,
}

impl<T: Data + Flowable, W: Widget<T>> LinkPoints<T, W> {
    pub fn new(inner: W) -> Self {
        LinkPoints {
            inner: WidgetPod::new(inner),
            points: Vec::new(),

            mouse_position: None,
        }
    }
}

impl<T: Data + Flowable, W: Widget<T>> Widget<T> for LinkPoints<T, W> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        match event {
            Event::MouseMove(mouse_event) => {
                self.mouse_position = Some(mouse_event.pos)
            },
            Event::MouseDown(mouse_event) => {
                if mouse_event.button.is_left() && mouse_event.count == 1 {
                    for (link_index, point) in self.points.iter().enumerate() {
                        let rect = link_point_rect(point);
                        if rect.contains(mouse_event.pos) {
                            ctx.submit_command(Command::new(LINK_STARTED, (data.get_id(), link_index), Target::Auto));
                            ctx.set_handled();
                            break;
                        }
                    }
                }
            },
            Event::MouseUp(mouse_event) => {
                if mouse_event.button.is_left() {
                    for (link_index, point) in self.points.iter().enumerate() {
                        let rect = link_point_rect(point);
                        if rect.contains(mouse_event.pos) {
                            ctx.submit_command(Command::new(LINK_FINISHED, (data.get_id(), link_index), Target::Auto));
                            ctx.set_handled();
                            break;
                        }
                    }

                    if !ctx.is_handled() {
                        // No matching link point. Just stop
                        ctx.submit_command(Command::new(LINK_STOPPED, (), Target::Auto));
                        ctx.set_handled();
                    }
                }
            },
            _ => { }
        }

        self.inner.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.inner.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {
        self.inner.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        let half_link_size = LINK_POINT_SIZE / 2.0;
        let inner_offset = Point::new(half_link_size, half_link_size);
        let inner_size = self.inner.layout(ctx, bc, data, env);
        self.inner.set_origin(ctx, data, env, inner_offset);

        self.points = data
            .get_link_points(inner_size)
            .into_iter()
            // Translate the link points to be centered on the inner widget
            .map(|link_point| link_point.with_offset(inner_offset))
            .collect();
        // Report a size 
        Size::new(inner_size.width + LINK_POINT_SIZE, inner_size.height + LINK_POINT_SIZE)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        self.inner.paint(ctx, data, env);

        if ctx.is_hot() {
            for point in self.points.iter() {
                let rect = link_point_rect(point);

                let mut color = theme::BORDER_LIGHT;
                if let Some(mouse_position) = self.mouse_position {
                    if rect.contains(mouse_position) {
                        color = theme::BORDER_DARK;
                    }
                }

                let rect = rect.to_rounded_rect(2.0);
                ctx.fill(rect, &env.get(color));
            }
        }
    }
}
