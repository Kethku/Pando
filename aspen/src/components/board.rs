use std::{collections::HashSet, ops::Deref};

use ordered_float::OrderedFloat;
use vello::{
    kurbo::{Affine, Circle, Point, Rect, Size},
    peniko::{Brush, Color},
};

use crate::{
    context_stack::{Context, DrawContext, LayoutContext, UpdateContext},
    element::{Element, ElementPointer},
    token::Token,
    util::*,
};

pub trait Pinnable: Element {
    fn center(&self, cx: &Context) -> Point;
}

pub struct Board {
    draw_background: Box<dyn Fn(Rect, &mut DrawContext)>,
    children: Vec<ElementPointer<Box<dyn Pinnable>>>,
}

#[derive(Default)]
pub struct BoardState {
    transform: Affine,
}

impl Board {
    pub fn new<'a>(
        transform: Affine,
        draw_background: impl Fn(Rect, &mut DrawContext) + 'static,
        cx: &Context<'a>,
    ) -> ElementPointer<Self> {
        ElementPointer::new(Self {
            draw_background: Box::new(draw_background),
            children: Vec::new(),
        })
        .insert_state(BoardState { transform }, cx)
    }

    pub fn new_dotgrid<'a>(
        transform: Affine,
        background: Color,
        dots: Color,
        cx: &Context<'a>,
    ) -> ElementPointer<Self> {
        let draw_background = move |bounds: Rect, cx: &mut DrawContext| {
            cx.set_fill_brush(Brush::Solid(background));
            cx.fill(&bounds);

            if bounds.is_zero_area() {
                return;
            }

            let mut spacing = 8192.;
            let mut filled_spaces = HashSet::new();
            loop {
                spacing = spacing / 2.;
                let mut radius = spacing / 50.;
                let scale = cx.current_transform().unskewed_scale().length() / 2.0f64.sqrt();
                let actual_radius = scale * radius;
                if actual_radius < 0.75 {
                    break;
                } else if actual_radius > 8. {
                    continue;
                }

                radius = radius.min(2. / scale);

                let mut x = bounds.min_x() - bounds.min_x().rem_euclid(spacing);
                loop {
                    let mut y = bounds.min_y() - bounds.min_y().rem_euclid(spacing);
                    loop {
                        cx.set_fill_brush(Brush::Solid(
                            background.mix(&dots, (actual_radius - 0.75) * 4.),
                        ));

                        let point = (OrderedFloat(x), OrderedFloat(y));
                        if !filled_spaces.contains(&point) {
                            cx.fill(&Circle::new(Point::new(x, y).snap(), radius));
                            filled_spaces.insert(point);
                        }

                        y += spacing;
                        if y > bounds.max_y() {
                            break;
                        }
                    }
                    x += spacing;
                    if x > bounds.max_x() {
                        break;
                    }
                }
            }
        };

        Self::new(transform, draw_background, cx)
    }

    pub fn add_child(&mut self, child: ElementPointer<impl Pinnable + 'static>) {
        self.children
            .push(child.map(|element| Box::new(element) as Box<dyn Pinnable + 'static>));
    }
}

impl ElementPointer<Board> {
    pub fn transform<'a>(&self, cx: &impl Deref<Target = Context<'a>>) -> Affine {
        self.with_state(cx, |state: &mut BoardState| state.transform)
    }
}

impl Element for Board {
    fn update(&mut self, cx: &mut UpdateContext) {
        for child in self.children.iter_mut() {
            child.update(cx);
        }
    }

    fn layout(&mut self, _min: Size, max: Size, cx: &mut LayoutContext) -> Size {
        let transform = cx.with_state(|state: &mut BoardState, _| {
            Affine::translate((max / 2.).to_vec2()) * state.transform
        });
        for child in self.children.iter_mut() {
            let result = child.layout(Size::ZERO, Size::INFINITY, cx);
            let position =
                child.with_context(cx, |cx| child.center(cx)) - result.size().to_vec2() / 2.;
            result.position(transform.pre_translate(position.to_vec2()), cx);
        }

        max
    }

    fn draw(&self, cx: &mut DrawContext) {
        let region = cx.region();
        let center = region.center().to_vec2();
        cx.mouse_region(region)
            .on_down(|cx| cx.focus())
            .on_right_drag(|cx| {
                if let Some(delta) = cx.mouse_delta() {
                    cx.with_state(|state: &mut BoardState, _| {
                        state.transform = state.transform.then_translate(delta);
                    });
                    cx.request_redraw();
                }
            })
            .on_scroll(move |cx| {
                if let Some(pos) = cx.mouse_position() {
                    let new_transform = cx.with_state(|state: &mut BoardState, cx| {
                        state
                            .transform
                            .then_scale_about(1.0 + cx.scroll_delta().y / 100.0, pos - center)
                    });

                    let test_length = new_transform.unskewed_scale().length() / 2.0f64.sqrt();
                    if test_length < 100. && test_length > 0.025 {
                        cx.with_state(|state: &mut BoardState, _| {
                            state.transform = new_transform;
                        });
                        cx.request_redraw();
                    }
                }
            });

        let window_region = cx
            .current_transform()
            .inverse()
            .transform_rect_bbox(Rect::from_origin_size(Point::ZERO, cx.window_size));
        let adjusted_transform =
            cx.with_state(|state: &mut BoardState, _| Affine::translate(center) * state.transform);
        let inverse_transform = adjusted_transform.inverse();
        let background = inverse_transform
            .transform_rect_bbox(region)
            .intersect(inverse_transform.transform_rect_bbox(window_region));

        cx.push_layer(1.0, &region);
        cx.transform(adjusted_transform);

        (self.draw_background)(background, cx);

        for child in self.children.iter() {
            child.draw(cx);
        }

        cx.pop_layer();
    }

    fn children(&self) -> Vec<Token> {
        self.children.iter().map(|c| c.tokens()).flatten().collect()
    }
}

pub struct PinWrapper<Child: Element> {
    child: ElementPointer<Child>,
    size: Option<Size>,
}

impl<Child: Element> PinWrapper<Child> {
    pub fn new(center: Point, child: ElementPointer<Child>, cx: &Context) -> ElementPointer<Self> {
        ElementPointer::new(Self { child, size: None }).insert_state(center, cx)
    }

    pub fn new_sized(
        center: Point,
        size: Size,
        child: ElementPointer<Child>,
        cx: &Context,
    ) -> ElementPointer<Self> {
        ElementPointer::new(Self {
            child,
            size: Some(size),
        })
        .insert_state(center, cx)
    }

    pub fn sized(mut this: ElementPointer<Self>, size: Size) -> ElementPointer<Self> {
        this.size = Some(size);
        this
    }
}

impl<Child: Element> Element for PinWrapper<Child> {
    fn update(&mut self, cx: &mut UpdateContext) {
        self.child.update(cx)
    }

    fn layout(&mut self, min: Size, max: Size, cx: &mut LayoutContext) -> Size {
        if let Some(size) = self.size {
            self.child
                .layout(size, size, cx)
                .position(Affine::IDENTITY, cx)
        } else {
            self.child
                .layout(min, max, cx)
                .position(Affine::IDENTITY, cx)
        }
    }

    fn draw(&self, cx: &mut DrawContext) {
        cx.mouse_region(cx.region())
            .on_down(|_| {
                // Block the base pin event from stealing focus
            })
            .on_drag({
            move |cx| {
                if let Some(delta) = cx.mouse_delta() {
                    cx.with_state(|center: &mut Point, _| {
                        *center += delta;
                    });
                    cx.request_redraw();
                }
            }
        });

        self.child.draw(cx);
    }
}

impl<Child: Element> Pinnable for PinWrapper<Child> {
    fn center(&self, cx: &Context) -> Point {
        cx.with_state(|center: &mut Point| *center)
    }
}

pub trait ElementPinExt<This: Element + Sized> {
    fn as_pinnable(self, center: Point, cx: &Context) -> ElementPointer<PinWrapper<This>>;
}

impl<This: Element + Sized> ElementPinExt<This> for ElementPointer<This> {
    fn as_pinnable(self, center: Point, cx: &Context) -> ElementPointer<PinWrapper<This>> {
        PinWrapper::new(center, self, cx)
    }
}
