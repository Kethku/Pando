use aspen::prelude::*;

use crate::util::*;

pub struct Todo {
    editor: ElementPointer<Border<TextEditor>>,
}

impl Todo {
    pub fn new(center: Point) -> ElementPointer<PinWrapper<Self>> {
        ElementPointer::new(Self {
            editor: TextEditor::new(Brush::Solid(*FOREGROUND)).with_border(
                10.,
                Brush::Solid(*BACKGROUND5),
                Brush::Solid(*BACKGROUND1),
            ),
        })
        .as_pinnable(center)
    }
}

impl Element for Todo {
    fn layout(&mut self, min: Size, max: Size, cx: &mut LayoutContext) -> Size {
        self.editor
            .layout(min, max, cx)
            .position(Affine::IDENTITY, cx)
    }

    fn draw(&self, cx: &mut DrawContext) {
        self.editor.draw(cx);
    }
}
