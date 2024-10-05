use glamour::prelude::*;
use palette::Srgba;
use vide::{
    parley::{style::StyleProperty, Layout},
    prelude::*,
};

use crate::{
    framework::{
        context::{ContextEventLoop, ContextWindow, DrawContext, LayoutContext},
        element::{Element, ElementPointer},
    },
    util::*,
};

pub struct TextEditor {
    shaper: Shaper,

    text: String,
    layout: Layout<Srgba>,
}

impl TextEditor {
    pub fn new() -> ElementPointer<Self> {
        let mut shaper = Shaper::new();
        shaper.push_default(StyleProperty::FontSize(16.));
        shaper.push_default(StyleProperty::Brush(*FOREGROUND));

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
    fn layout(&mut self, min: Size2, max: Size2, _cx: &mut LayoutContext) -> Size2 {
        let size = size2!(self.layout.width(), self.layout.height());
        let size = size.clamp(min, max);
        size
    }

    fn draw(&self, cx: &mut DrawContext) {
        let top_left = cx.region().origin;
        cx.update_layer(|resources, layer| {
            layer.add_text_layout(resources, &self.layout, top_left);
        });
    }
}
