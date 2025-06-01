use std::{cell::RefCell, collections::HashSet, rc::Rc};

use ordered_float::OrderedFloat;
use vello::{
    kurbo::{Affine, Circle, Point, Rect, Size},
    peniko::{Brush, Color},
};

use crate::{
    context::{DrawContext, LayoutContext, UpdateContext},
    element::{Element, ElementPointer},
    util::*,
};

pub trait Pinnable: Element {
    fn center(&self) -> Point;
}

pub struct Board<Child: Pinnable> {
    draw_background: Box<dyn Fn(Rect, &mut DrawContext)>,
    children: Vec<ElementPointer<Child>>,

    transform: Rc<RefCell<Affine>>,
}

impl<Child: Pinnable> Board<Child> {
    pub fn new(
        transform: Affine,
        draw_background: impl Fn(Rect, &mut DrawContext) + 'static,
    ) -> ElementPointer<Self> {
        ElementPointer::new(Self {
            draw_background: Box::new(draw_background),
            children: Vec::new(),

            transform: Rc::new(RefCell::new(transform)),
        })
    }

    pub fn new_dotgrid(transform: Affine, background: Color, dots: Color) -> ElementPointer<Self> {
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

        Self::new(transform, draw_background)
    }

    pub fn update_transform(&mut self, update: impl FnOnce(Affine) -> Affine) {
        let transform = *self.transform.borrow();
        *self.transform.borrow_mut() = update(transform);
    }

    pub fn add_child(&mut self, child: impl Into<ElementPointer<Child>>) {
        self.children.push(child.into());
    }

    pub fn transform(&self) -> Affine {
        *self.transform.borrow()
    }
}

impl<Child: Pinnable> Element for Board<Child> {
    fn update(&mut self, cx: &mut UpdateContext) {
        for child in self.children.iter_mut() {
            child.update(cx);
        }
    }

    fn layout(&mut self, _min: Size, max: Size, cx: &mut LayoutContext) -> Size {
        let transform = self.transform();
        for child in self.children.iter_mut() {
            let result = child.layout(Size::ZERO, Size::INFINITY, cx);
            let position = child.center() - result.size().to_vec2() / 2.;
            result.position(transform * Affine::translate(position.to_vec2()), cx);
        }

        max
    }

    fn draw(&self, cx: &mut DrawContext) {
        cx.mouse_region(cx.region())
            .on_right_drag({
                let transform = self.transform.clone();
                move |cx| {
                    if let Some(delta) = cx.mouse_delta() {
                        let mut transform = transform.borrow_mut();
                        *transform = transform.then_translate(delta);
                        cx.request_redraw();
                    }
                }
            })
            .on_scroll({
                let transform = self.transform.clone();
                move |cx| {
                    if let Some(pos) = cx.mouse_position() {
                        let mut transform = transform.borrow_mut();
                        let new_transform =
                            transform.then_scale_about(1.0 + cx.scroll_delta().y / 100.0, pos);

                        let test_length = new_transform.unskewed_scale().length() / 2.0f64.sqrt();
                        if test_length < 100. && test_length > 0.025 {
                            *transform = new_transform;
                            cx.request_redraw();
                        }
                    }
                }
            });

        let window_region = cx
            .current_transform()
            .inverse()
            .transform_rect_bbox(Rect::from_origin_size(Point::ZERO, cx.window_size));
        let region = cx.region();
        let inverse_transform = self.transform().inverse();
        let background = inverse_transform
            .transform_rect_bbox(region)
            .intersect(inverse_transform.transform_rect_bbox(window_region));

        cx.push_layer(1.0, &region);
        cx.transform(self.transform());

        (self.draw_background)(background, cx);

        for child in self.children.iter() {
            child.draw(cx);
        }

        cx.pop_layer();
    }
}

pub struct PinWrapper<Child: Element> {
    child: ElementPointer<Child>,

    size: Option<Size>,
    center: Rc<RefCell<Point>>,
}

impl<Child: Element> PinWrapper<Child> {
    pub fn new(center: Point, child: ElementPointer<Child>) -> ElementPointer<Self> {
        ElementPointer::new(Self {
            child,
            size: None,
            center: Rc::new(RefCell::new(center)),
        })
    }

    pub fn new_sized(
        center: Point,
        size: Size,
        child: ElementPointer<Child>,
    ) -> ElementPointer<Self> {
        ElementPointer::new(Self {
            child,
            size: Some(size),
            center: Rc::new(RefCell::new(center)),
        })
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
        cx.mouse_region(cx.region()).on_drag({
            let center = self.center.clone();
            move |cx| {
                if let Some(delta) = cx.mouse_delta() {
                    let mut center = center.borrow_mut();
                    *center += delta;
                    cx.request_redraw();
                }
            }
        });

        self.child.draw(cx);
    }
}

impl<Child: Element> Pinnable for PinWrapper<Child> {
    fn center(&self) -> Point {
        *self.center.borrow()
    }
}

pub trait ElementPinExt<This: Element + Sized> {
    fn as_pinnable(self, center: Point) -> ElementPointer<PinWrapper<This>>;
}

impl<This: Element + Sized> ElementPinExt<This> for ElementPointer<This> {
    fn as_pinnable(self, center: Point) -> ElementPointer<PinWrapper<This>> {
        PinWrapper::new(center, self)
    }
}
