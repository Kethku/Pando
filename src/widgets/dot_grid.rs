use druid::piet::kurbo::Circle;
use druid::theme;
use druid::widget::Painter;
use druid::widget::prelude::*;

use crate::controllers::draggable::Positioned;

const BACKGROUND_GRID_SIZE: isize = 25;
const BACKGROUND_CIRCLE_RADIUS: f64 = 1.0;

pub fn dot_grid<T: Data + Positioned>() -> Painter<T> {
    Painter::new(|ctx, data: &T, env| {
        let size = ctx.size();
        let offset = data.get_position();

        let rect = size.to_rect();
        ctx.fill(rect, &env.get(theme::BACKGROUND_DARK));

        for x in (-BACKGROUND_GRID_SIZE..(size.width.ceil() as isize + BACKGROUND_GRID_SIZE)).step_by(BACKGROUND_GRID_SIZE as usize) {
            for y in (-BACKGROUND_GRID_SIZE..(size.height.ceil() as isize + BACKGROUND_GRID_SIZE)).step_by(BACKGROUND_GRID_SIZE as usize) {
                let circle = Circle::new(
                    (x as f64 + offset.x % BACKGROUND_GRID_SIZE as f64, 
                     y as f64 + offset.y % BACKGROUND_GRID_SIZE as f64), 
                    BACKGROUND_CIRCLE_RADIUS);
                ctx.fill(circle, &env.get(theme::BORDER_LIGHT));
            }
        }
    })
}
