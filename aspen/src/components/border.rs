use vello::{
    kurbo::{Affine, Size, Stroke, Vec2},
    peniko::{Brush, Color},
};

use crate::{
    context_stack::{DrawContext, LayoutContext, UpdateContext},
    element::{Element, ElementPointer},
    token::Token,
};

pub struct Border<Child: Element> {
    pub child: ElementPointer<Child>,

    pub padding: f64,
    pub stroke: Brush,
    pub fill: Brush,
    pub thickness: f64,
    pub radius: f64,
    pub background_separation: f64,
}

impl<Child: Element> Border<Child> {
    pub fn new(
        child: ElementPointer<Child>,
        padding: f64,
        stroke: Brush,
        fill: Brush,
    ) -> ElementPointer<Self> {
        ElementPointer::new(Self {
            child,

            padding,
            stroke,
            fill,
            thickness: 2.,
            radius: 5.,
            background_separation: 2.,
        })
    }

    pub fn with_thickness(mut this: ElementPointer<Self>, thickness: f64) -> ElementPointer<Self> {
        this.thickness = thickness;
        this
    }

    pub fn with_radius(mut this: ElementPointer<Self>, radius: f64) -> ElementPointer<Self> {
        this.radius = radius;
        this
    }

    pub fn with_background_separation(
        mut this: ElementPointer<Self>,
        background_separation: f64,
    ) -> ElementPointer<Self> {
        this.background_separation = background_separation;
        this
    }
}

impl<Child: Element> Element for Border<Child> {
    fn update(&mut self, cx: &mut UpdateContext) {
        self.child.update(cx)
    }

    fn layout(&mut self, min: Size, max: Size, cx: &mut LayoutContext) -> Size {
        let padding_adjustment = Size::new(self.padding * 2., self.padding * 2.);
        let child_result =
            self.child
                .layout(min - padding_adjustment, max - padding_adjustment, cx);

        (child_result.position(Affine::translate((self.padding, self.padding)), cx)
            + padding_adjustment)
            .clamp(min, max)
    }

    fn draw(&self, cx: &mut DrawContext) {
        let region = cx.region().to_rounded_rect(self.radius);
        if self.background_separation > 0. {
            cx.set_fill_brush(Brush::Solid(Color::new([0., 0., 0., 0.5])));
            cx.blurred(
                region + Vec2::new(0., self.background_separation),
                self.background_separation * 2.,
            );
        }

        cx.set_fill_brush(self.fill.clone());
        cx.set_stroke_brush(self.stroke.clone());
        cx.set_stroke_style(Stroke::new(self.thickness));
        cx.stroked_fill(&region);

        self.child.draw(cx);
    }

    fn children(&self) -> Vec<Token> {
        self.child.tokens()
    }
}

pub trait ElementBorderExt<This: Element + Sized> {
    fn with_border(self, padding: f64, stroke: Brush, fill: Brush) -> ElementPointer<Border<This>>;
}

impl<This: Element + Sized> ElementBorderExt<This> for ElementPointer<This> {
    fn with_border(self, padding: f64, stroke: Brush, fill: Brush) -> ElementPointer<Border<This>> {
        Border::new(self, padding, stroke, fill)
    }
}
