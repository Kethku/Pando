use glamour::prelude::*;
use palette::Srgba;
use vide::*;

use super::button::Button;
use crate::{
    framework::{
        context::{DrawContext, LayoutContext, UpdateContext},
        element::{Element, ElementPointer},
        mouse_region::MouseRegion,
    },
    util::*,
};

const TITLEBAR_HEIGHT: f32 = 34.;
const BUTTON_ASPECT_RATIO: f32 = 1.666;
const X_HEIGHT: f32 = 10.;

pub struct WindowButtons {
    close: ElementPointer<Button>,
    maximize: ElementPointer<Button>,
    minimize: ElementPointer<Button>,
}

impl WindowButtons {
    pub fn new() -> ElementPointer<Self> {
        let button_size = size2!(TITLEBAR_HEIGHT * BUTTON_ASPECT_RATIO, TITLEBAR_HEIGHT);
        ElementPointer::new(Self {
            close: Button::new(
                button_size,
                Srgba::new(0., 0., 0., 0.),
                *CLOSE,
                |rect, cx| Self::draw_close_icon(rect, cx),
                |cx| cx.close(),
            ),
            maximize: Button::new(
                button_size,
                Srgba::new(0., 0., 0., 0.),
                *BACKGROUND3,
                |rect, cx| Self::draw_maximize_icon(rect, cx),
                |cx| cx.toggle_maximized(),
            ),
            minimize: Button::new(
                button_size,
                Srgba::new(0., 0., 0., 0.),
                *BACKGROUND3,
                |rect, cx| Self::draw_minimize_icon(rect, cx),
                |cx| cx.minimize(),
            ),
        })
    }

    fn draw_close_icon(rect: Rect, cx: &mut DrawContext) {
        cx.update_layer(|_, layer| {
            layer.add_path(
                Path::new_line(
                    1.,
                    *FOREGROUND,
                    rect.center() - vector!(X_HEIGHT / 2., X_HEIGHT / 2.),
                )
                .with_line_to(rect.center() + vector!(X_HEIGHT / 2., X_HEIGHT / 2.)),
            );
            layer.add_path(
                Path::new_line(
                    1.,
                    *FOREGROUND,
                    rect.center() + vector!(-X_HEIGHT / 2., X_HEIGHT / 2.),
                )
                .with_line_to(rect.center() + vector!(X_HEIGHT / 2., -X_HEIGHT / 2.)),
            );
        });
    }

    fn draw_maximize_icon(rect: Rect, cx: &mut DrawContext) {
        const RESTORE_OFFSET: f32 = 3.;
        let icon_rect = Rect::new(
            rect.center() - vector!(X_HEIGHT / 2., X_HEIGHT / 2.),
            size2!(X_HEIGHT, X_HEIGHT),
        );
        let icon_corners = icon_rect.corners();

        if cx.is_maximized() {
            cx.update_layer(|_, layer| {
                // Outer border
                layer.add_path(
                    Path::new_line(
                        1.,
                        *FOREGROUND,
                        icon_corners[0] + vector!(RESTORE_OFFSET, 0.),
                    )
                    .with_line_to(icon_corners[1])
                    .with_line_to(icon_corners[2] + vector!(0., -RESTORE_OFFSET)),
                );

                // Inner rect
                layer.add_path(
                    Path::new_stroke(
                        1.,
                        *FOREGROUND,
                        icon_corners[0] + vector!(0., RESTORE_OFFSET),
                    )
                    .with_line_to(icon_corners[1] + vector!(-RESTORE_OFFSET, RESTORE_OFFSET))
                    .with_line_to(icon_corners[2] + vector!(-RESTORE_OFFSET, 0.))
                    .with_line_to(icon_corners[3]),
                );
            });
        } else {
            cx.update_layer(|_, layer| {
                layer.add_path(
                    Path::new_stroke(1., *FOREGROUND, icon_corners[0])
                        .with_line_to(icon_corners[1])
                        .with_line_to(icon_corners[2])
                        .with_line_to(icon_corners[3]),
                );
            });
        }
    }

    fn draw_minimize_icon(rect: Rect, cx: &mut DrawContext) {
        let icon_rect = Rect::new(
            rect.center() - vector!(X_HEIGHT / 2., X_HEIGHT / 2.),
            size2!(X_HEIGHT, X_HEIGHT),
        );

        cx.update_layer(|_, layer| {
            layer.add_path(
                Path::new_line(
                    1.,
                    *FOREGROUND,
                    icon_rect.center() + vector!(-icon_rect.width() / 2., 0.),
                )
                .with_line_to(icon_rect.center() + vector!(icon_rect.width() / 2., 0.)),
            );
        });
    }
}

impl Element for WindowButtons {
    fn update(&mut self, cx: &mut UpdateContext) {
        self.close.update(cx);
        self.maximize.update(cx);
        self.minimize.update(cx);
    }

    fn layout(&mut self, min: Size2, max: Size2, cx: &mut LayoutContext) -> Size2 {
        let mut current_x = max.width;
        let mut height: f32 = 0.;

        let close_result = self.close.layout(min, max, cx);
        current_x -= close_result.size().width;
        height = height.max(close_result.size().height);
        close_result.position(point2!(current_x, 0.), cx);

        let maximize_result = self.maximize.layout(min, max, cx);
        current_x -= maximize_result.size().width;
        height = height.max(maximize_result.size().height);
        maximize_result.position(point2!(current_x, 0.), cx);

        let minimize_result = self.minimize.layout(min, max, cx);
        current_x -= minimize_result.size().width;
        height = height.max(minimize_result.size().height);
        minimize_result.position(point2!(current_x, 0.), cx);

        size2!(max.width, height)
    }

    fn draw(&self, cx: &mut DrawContext) {
        cx.add_layer(Layer::new());

        let region = cx.region();
        cx.update_layer(|_, layer| {
            layer.add_quad(Quad::new(region, *BACKGROUND2));
        });

        cx.add_mouse_region(MouseRegion::new(cx.token(), region).on_down({
            move |cx| {
                cx.drag_window();
            }
        }));

        self.close.draw(cx);
        self.maximize.draw(cx);
        self.minimize.draw(cx);
    }
}
