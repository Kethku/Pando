use vide::prelude::*;

use super::button::Button;
use crate::{
    context::{DrawContext, LayoutContext, UpdateContext},
    element::{Element, ElementPointer},
    mouse_region::MouseRegion,
};

const TITLEBAR_HEIGHT: f32 = 34.;
const BUTTON_ASPECT_RATIO: f32 = 1.666;
const X_HEIGHT: f32 = 10.;

pub struct WindowButtons {
    title_background: Srgba,

    close: ElementPointer<Button>,
    maximize: ElementPointer<Button>,
    minimize: ElementPointer<Button>,
}

impl WindowButtons {
    pub fn new(
        title_background: Srgba,
        close_hover: Srgba,
        foreground: Srgba,
    ) -> ElementPointer<Self> {
        let button_size = size2!(TITLEBAR_HEIGHT * BUTTON_ASPECT_RATIO, TITLEBAR_HEIGHT);
        ElementPointer::new(Self {
            title_background,

            close: Button::new(
                button_size,
                Srgba::new(0., 0., 0., 0.),
                close_hover,
                move |rect, cx| Self::draw_close_icon(rect, foreground, cx),
                |cx| cx.close(),
            ),
            maximize: Button::new(
                button_size,
                Srgba::new(0., 0., 0., 0.),
                title_background,
                move |rect, cx| Self::draw_maximize_icon(rect, foreground, cx),
                |cx| cx.toggle_maximized(),
            ),
            minimize: Button::new(
                button_size,
                Srgba::new(0., 0., 0., 0.),
                title_background,
                move |rect, cx| Self::draw_minimize_icon(rect, foreground, cx),
                |cx| cx.minimize(),
            ),
        })
    }

    fn draw_close_icon(rect: Rect, foreground: Srgba, cx: &mut DrawContext) {
        cx.update_layer(|_, layer| {
            layer.add_path(
                Path::new_line(
                    1.,
                    foreground,
                    rect.center() - vector!(X_HEIGHT / 2., X_HEIGHT / 2.),
                )
                .with_line_to(rect.center() + vector!(X_HEIGHT / 2., X_HEIGHT / 2.)),
            );
            layer.add_path(
                Path::new_line(
                    1.,
                    foreground,
                    rect.center() + vector!(-X_HEIGHT / 2., X_HEIGHT / 2.),
                )
                .with_line_to(rect.center() + vector!(X_HEIGHT / 2., -X_HEIGHT / 2.)),
            );
        });
    }

    fn draw_maximize_icon(rect: Rect, foreground: Srgba, cx: &mut DrawContext) {
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
                        foreground,
                        icon_corners[0] + vector!(RESTORE_OFFSET, 0.),
                    )
                    .with_line_to(icon_corners[1])
                    .with_line_to(icon_corners[2] + vector!(0., -RESTORE_OFFSET)),
                );

                // Inner rect
                layer.add_path(
                    Path::new_stroke(
                        1.,
                        foreground,
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
                    Path::new_stroke(1., foreground, icon_corners[0])
                        .with_line_to(icon_corners[1])
                        .with_line_to(icon_corners[2])
                        .with_line_to(icon_corners[3]),
                );
            });
        }
    }

    fn draw_minimize_icon(rect: Rect, foreground: Srgba, cx: &mut DrawContext) {
        let icon_rect = Rect::new(
            rect.center() - vector!(X_HEIGHT / 2., X_HEIGHT / 2.),
            size2!(X_HEIGHT, X_HEIGHT),
        );

        cx.update_layer(|_, layer| {
            layer.add_path(
                Path::new_line(
                    1.,
                    foreground,
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

    fn layout(&mut self, _min: Size2, max: Size2, cx: &mut LayoutContext) -> Size2 {
        let mut current_x = max.width;
        let button_size = size2!(TITLEBAR_HEIGHT * BUTTON_ASPECT_RATIO, TITLEBAR_HEIGHT);

        let close_result = self.close.layout(button_size, button_size, cx);
        current_x -= close_result.size().width;
        close_result.position(point2!(current_x, 0.), cx);

        let maximize_result = self.maximize.layout(button_size, button_size, cx);
        current_x -= maximize_result.size().width;
        maximize_result.position(point2!(current_x, 0.), cx);

        let minimize_result = self.minimize.layout(button_size, button_size, cx);
        current_x -= minimize_result.size().width;
        minimize_result.position(point2!(current_x, 0.), cx);

        size2!(max.width, TITLEBAR_HEIGHT)
    }

    fn draw(&self, cx: &mut DrawContext) {
        cx.add_layer(Layer::new());

        let region = cx.region();
        cx.update_layer(|_, layer| {
            layer.add_quad(Quad::new(region, self.title_background));
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
