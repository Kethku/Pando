use druid::Color;
use druid::kurbo::Circle;
use druid::widget::Painter;
use druid::widget::prelude::*;

pub fn dots<T: Data>(spacing: usize, radius: f64, background: Color, fill: Color) -> Painter<T> {
    Painter::new(move |ctx, _, _| {
        let size = ctx.size();
        let rect = size.to_rect();
        ctx.fill(rect, &background);

        for x in (0..(size.width.ceil() as usize)).step_by(spacing) {
            for y in (0..(size.height.ceil() as usize)).step_by(spacing) {
                let circle = Circle::new((x as f64, y as f64), radius);
                ctx.fill(circle, &fill);
            }
        }
    })
}
