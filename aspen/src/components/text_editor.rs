use parley::{style::StyleProperty, Layout};
use vello::{kurbo::Size, peniko::Brush};

use crate::{
    context::{DrawContext, LayoutContext},
    element::{Element, ElementPointer},
    shaper::Shaper,
};

pub struct TextEditor {
    shaper: Shaper,

    text: String,
    layout: Layout<Brush>,
}

impl TextEditor {
    pub fn new(foreground: Brush) -> ElementPointer<Self> {
        let mut shaper = Shaper::new();
        shaper.push_default(StyleProperty::FontSize(16.));
        shaper.push_default(StyleProperty::Brush(foreground));

        let text = "This is a test".to_string();
        let layout = shaper.layout(&text);

        ElementPointer::new(Self {
            shaper,

            text,
            layout,
        })
    }
}

impl Element for TextEditor {
    fn layout(&mut self, min: Size, max: Size, _cx: &mut LayoutContext) -> Size {
        let size = Size::new(self.layout.width() as f64, self.layout.height() as f64);
        let size = size.clamp(min, max);
        size
    }

    fn draw(&self, cx: &mut DrawContext) {
        let top_left = cx.region().origin();
        cx.draw_layout(&self.layout, top_left);
    }
}
