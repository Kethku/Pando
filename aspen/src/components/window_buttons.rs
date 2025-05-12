use vello::{
    kurbo::{BezPath, Line, Point, Rect, Size, Stroke, Vec2},
    peniko::{Brush, Color},
};

use crate::{
    context::{DrawContext, LayoutContext, UpdateContext},
    element::{Element, ElementPointer},
    util::{PointExt, RectExt},
};

use super::button::Button;

const TITLEBAR_HEIGHT: f64 = 34.;
const BUTTON_ASPECT_RATIO: f64 = 1.666;
const ICON_SHIFT: Vec2 = Vec2::new(0., -1.);
const X_HEIGHT: f64 = 10.;

pub struct WindowButtons {
    title_background: Color,

    close: ElementPointer<Button>,
    maximize: ElementPointer<Button>,
    minimize: ElementPointer<Button>,
}

impl WindowButtons {
    pub fn new(
        title_background: Color,
        close_hover: Color,
        other_hover: Color,
        foreground: Color,
    ) -> ElementPointer<Self> {
        let button_size = Size::new(TITLEBAR_HEIGHT * BUTTON_ASPECT_RATIO, TITLEBAR_HEIGHT);
        ElementPointer::new(Self {
            title_background,

            close: Button::new(
                button_size,
                Color::new([0., 0., 0., 0.]),
                close_hover,
                move |rect, cx| Self::draw_close_icon(rect, foreground, cx),
                |cx| cx.close(),
            ),
            maximize: Button::new(
                button_size,
                Color::new([0., 0., 0., 0.]),
                other_hover,
                move |rect, cx| Self::draw_maximize_icon(rect, foreground, cx),
                |cx| cx.toggle_maximized(),
            ),
            minimize: Button::new(
                button_size,
                Color::new([0., 0., 0., 0.]),
                other_hover,
                move |rect, cx| Self::draw_minimize_icon(rect, foreground, cx),
                |cx| cx.minimize(),
            ),
        })
    }

    fn icon_rect(rect: Rect) -> Rect {
        Rect::from_origin_size(
            (rect.center() - Vec2::new(X_HEIGHT / 2., X_HEIGHT / 2.)).snap(),
            Size::new(X_HEIGHT, X_HEIGHT),
        ) + ICON_SHIFT
    }

    fn draw_close_icon(rect: Rect, foreground: Color, cx: &mut DrawContext) {
        let icon_rect = Self::icon_rect(rect);
        let corners = icon_rect.corners();

        cx.set_stroke_style(Stroke::new(1.));
        cx.set_stroke_brush(Brush::Solid(foreground));

        cx.stroke(&Line::new(corners[0], corners[2]));
        cx.stroke(&Line::new(corners[1], corners[3]));
    }

    fn draw_maximize_icon(rect: Rect, foreground: Color, cx: &mut DrawContext) {
        const RESTORE_OFFSET: f64 = 2.;
        let icon_rect = Self::icon_rect(rect);
        let corners = icon_rect.corners();
        cx.set_stroke_style(Stroke::new(1.));
        cx.set_stroke_brush(Brush::Solid(foreground));

        if cx.is_maximized() {
            let mut path = BezPath::new();
            path.move_to(corners[0] + Vec2::new(RESTORE_OFFSET, 0.));
            path.line_to(corners[1]);
            path.line_to(corners[2] + Vec2::new(0., -RESTORE_OFFSET));
            cx.stroke(&path);
            cx.stroke(&Rect::from_points(
                corners[1] + Vec2::new(-RESTORE_OFFSET, RESTORE_OFFSET),
                corners[3],
            ));
        } else {
            cx.stroke(&Rect::from_points(corners[0], corners[2]));
        }
    }

    fn draw_minimize_icon(rect: Rect, foreground: Color, cx: &mut DrawContext) {
        let icon_rect = Self::icon_rect(rect);
        cx.set_stroke_style(Stroke::new(1.));
        cx.set_stroke_brush(Brush::Solid(foreground));

        cx.stroke(&Line::new(
            icon_rect.center_left(),
            icon_rect.center_right(),
        ));
    }
}

impl Element for WindowButtons {
    fn update(&mut self, cx: &mut UpdateContext) {
        self.close.update(cx);
        self.maximize.update(cx);
        self.minimize.update(cx);
    }

    fn layout(&mut self, _min: Size, max: Size, cx: &mut LayoutContext) -> Size {
        let mut current_x = max.width;
        let button_size = Size::new(TITLEBAR_HEIGHT * BUTTON_ASPECT_RATIO, TITLEBAR_HEIGHT);

        let close_result = self.close.layout(button_size, button_size, cx);
        current_x -= close_result.size().width;
        close_result.position(Point::new(current_x, 0.), cx);

        let maximize_result = self.maximize.layout(button_size, button_size, cx);
        current_x -= maximize_result.size().width;
        maximize_result.position(Point::new(current_x, 0.), cx);

        let minimize_result = self.minimize.layout(button_size, button_size, cx);
        current_x -= minimize_result.size().width;
        minimize_result.position(Point::new(current_x, 0.), cx);

        Size::new(max.width, TITLEBAR_HEIGHT)
    }

    fn draw(&self, cx: &mut DrawContext) {
        cx.set_fill_brush(Brush::Solid(self.title_background));

        let region = cx.region();
        cx.fill(&region);

        cx.mouse_region(region).on_down({
            move |cx| {
                cx.drag_window();
            }
        });

        self.close.draw(cx);
        self.maximize.draw(cx);
        self.minimize.draw(cx);
    }
}
