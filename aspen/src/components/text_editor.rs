use parley::{
    layout::{
        cursor::{Cursor, Selection},
        Alignment,
    },
    style::StyleProperty,
    Layout,
};
use vello::{kurbo::Size, peniko::Brush};

use crate::{
    context_stack::{DrawContext, LayoutContext},
    element::{Element, ElementPointer},
};

/// Opaque representation of a generation.
///
/// Obtained from [`PlainEditor::generation`].
// Overflow handling: the generations are only compared,
// so wrapping is fine. This could only fail if exactly
// `u32::MAX` generations happen between drawing
// operations. This is implausible and so can be ignored.
#[derive(PartialEq, Eq, Default, Clone, Copy)]
pub struct Generation(u32);

impl Generation {
    /// Make it not what it currently is.
    pub(crate) fn nudge(&mut self) {
        self.0 = self.0.wrapping_add(1);
    }
}

pub struct TextEditor {
    text: String,
    layout: Option<Layout<Brush>>,
    foreground: Brush,
    selection: Selection,
    width: Option<f64>,
    layout_dirty: bool,
    alignment: Alignment,
    generation: Generation,
}

impl TextEditor {
    pub fn new(foreground: Brush) -> ElementPointer<Self> {
        let text = "This is a test".to_string();

        ElementPointer::new(Self {
            text,
            layout: None,
            foreground,
            selection: Default::default(),
            width: None,
            layout_dirty: true,
            alignment: Alignment::Start,
            generation: Generation(1),
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
        cx.mouse_region(cx.region()).on_click(|cx| cx.focus());

        let top_left = cx.region().origin();
        if let Some(layout) = &self.layout {
            cx.draw_layout_at(layout, top_left);
        }
    }
}
