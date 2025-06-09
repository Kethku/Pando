use parley::{style::StyleProperty, Layout};
use vello::{kurbo::Size, peniko::Brush};

use crate::{
    context_stack::{DrawContext, LayoutContext},
    element::{Element, ElementPointer},
};

pub struct TextEditor {
    text: String,
    foreground: Brush,
    layout: Option<Layout<Brush>>,
}

impl TextEditor {
    pub fn new(foreground: Brush) -> ElementPointer<Self> {
        let text = "This is a test".to_string();

        ElementPointer::new(Self {
            text,
            foreground,
            layout: None,
        })
    }
}

impl Element for TextEditor {
    fn layout(&mut self, min: Size, max: Size, cx: &mut LayoutContext) -> Size {
        let layout = self.layout.get_or_insert_with(|| {
            let foreground = self.foreground.clone();
            cx.push_default_text_style(StyleProperty::FontSize(16.));
            cx.push_default_text_style(StyleProperty::Brush(foreground));
            cx.layout(&self.text)
        });

        let size = Size::new(layout.width() as f64, layout.height() as f64);
        let size = size.clamp(min, max);
        size
    }

    fn draw(&self, cx: &mut DrawContext) {
        let top_left = cx.region().origin();
        if let Some(layout) = &self.layout {
            cx.draw_layout_at(layout, top_left);
        }
    }
}
