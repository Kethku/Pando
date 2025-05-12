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

            let mut spacing = 2048.;
            let mut filled_spaces = HashSet::new();
            loop {
                spacing = spacing / 2.;
                let mut radius = spacing / 50.;
                let scale = cx.current_transform().unskewed_scale().length() / 2.0f64.sqrt();
                let actual_radius = scale * radius;
                if actual_radius < 0.25 {
                    break;
                } else if actual_radius > 4. {
                    continue;
                }

                radius = radius.min(2. / scale);

                let mut x = bounds.min_x() - bounds.min_x().rem_euclid(spacing);
                loop {
                    let mut y = bounds.min_y() - bounds.min_y().rem_euclid(spacing);
                    loop {
                        cx.set_fill_brush(Brush::Solid(
                            background.mix(&dots, (actual_radius - 0.5) * 4.),
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

    pub fn add_child(&mut self, child: ElementPointer<Child>) {
        self.children.push(child);
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
        for child in self.children.iter_mut() {
            let result = child.layout(Size::ZERO, Size::INFINITY, cx);
            let position = child.center() - result.size().to_vec2() / 2.;
            result.position(position, cx);
        }

        max
    }

    fn draw(&self, cx: &mut DrawContext) {
        cx.mouse_region(cx.region())
            .on_drag({
                let transform = self.transform.clone();
                move |_down, cx| {
                    let mut transform = transform.borrow_mut();
                    *transform = transform.then_translate(cx.mouse_delta());
                    cx.request_redraw();
                }
            })
            .on_scroll({
                let transform = self.transform.clone();
                move |cx| {
                    let mut transform = transform.borrow_mut();
                    let new_transform = transform
                        .then_scale_about(1.0 + cx.scroll_delta().y / 100.0, cx.mouse_position());
                    let test_length = ((new_transform * Point::new(1., 1.))
                        - (new_transform * Point::new(0., 0.)))
                    .length();
                    if test_length < 1000. && test_length > 0.1 {
                        *transform = new_transform;
                        cx.request_redraw();
                    }
                }
            });

        let region = cx.region();
        let background = self.transform().inverse().transform_rect_bbox(region);

        cx.push_layer(1.0, &cx.region());
        cx.transform(self.transform());

        (self.draw_background)(background, cx);

        for child in self.children.iter() {
            child.draw(cx);
        }

        cx.pop_layer();
    }
}
