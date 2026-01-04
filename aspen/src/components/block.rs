use vello::{
    kurbo::{Affine, Size, Stroke, Vec2},
    peniko::{Brush, Color},
};

use crate::{
    context_stack::{DrawContext, LayoutContext, UpdateContext},
    element::{Element, ElementPointer},
    token::Token,
};

pub struct BlockStyle {
    pub padding: f64,
    pub stroke: Brush,
    pub fill: Brush,
    pub border: f64,
    pub radius: f64,
    pub separation: f64,
}

pub struct Block<Child: Element> {
    pub child: ElementPointer<Child>,
    pub style: BlockStyle,
}

impl<Child: Element> Block<Child> {
    pub fn new(
        child: ElementPointer<Child>,
        style: BlockStyle,
    ) -> ElementPointer<Self> {
        ElementPointer::new(Self {
            child,
            style,
        })
    }
}

impl<Child: Element> Element for Block<Child> {
    fn update(&mut self, cx: &mut UpdateContext) {
        self.child.update(cx)
    }

    fn layout(&mut self, min: Size, max: Size, cx: &mut LayoutContext) -> Size {
        let padding_adjustment = Size::new(self.style.padding * 2., self.style.padding * 2.);
        let child_result =
            self.child
                .layout(min - padding_adjustment, max - padding_adjustment, cx);

        (child_result.position(Affine::translate((self.style.padding, self.style.padding)), cx)
            + padding_adjustment)
            .clamp(min, max)
    }

    fn draw(&self, cx: &mut DrawContext) {
        let region = cx.region().to_rounded_rect(self.style.radius);
        if self.style.separation > 0. {
            cx.set_fill_brush(Brush::Solid(Color::new([0., 0., 0., 0.5])));
            cx.blurred(
                region + Vec2::new(0., self.style.separation),
                self.style.separation * 2.,
            );
        }

        cx.set_fill_brush(self.style.fill.clone());
        cx.set_stroke_brush(self.style.stroke.clone());
        cx.set_stroke_style(Stroke::new(self.style.border));
        cx.stroked_fill(&region);

        self.child.draw(cx);
    }

    fn children(&self) -> Vec<Token> {
        self.child.tokens()
    }
}

pub trait ElementBlockExt<This: Element + Sized> {
    fn within(self, block_style: BlockStyle) -> ElementPointer<Block<This>>;
}

impl<This: Element + Sized> ElementBlockExt<This> for ElementPointer<This> {
    fn within(self, block_style: BlockStyle) -> ElementPointer<Block<This>> {
        Block::new(self, block_style)
    }
}
