use aspen::prelude::*;

use crate::util::*;

pub struct Todo {
    editor: ElementPointer<Border<TextEditor>>,
}

impl Todo {
    pub fn new() -> ElementPointer<Self> {
        Self {
            editor: TextEditor::new(Brush::Solid(*FOREGROUND)).with_border(
                10.,
                Brush::Solid(*BACKGROUND5),
                Brush::Solid(*BACKGROUND1),
            ),
        }
        .into()
    }
}

impl Element for Todo {
    fn update(&mut self, cx: &mut UpdateContext) {
        self.editor.fill = if cx.is_focused() {
            Brush::Solid(*BACKGROUND4)
        } else {
            Brush::Solid(*BACKGROUND5)
        }
    }

    fn layout(&mut self, min: Size, max: Size, cx: &mut LayoutContext) -> Size {
        self.editor
            .layout(min, max, cx)
            .position(Affine::translate(Vec2::new(0., 0.)), cx)
    }

    fn draw(&self, cx: &mut DrawContext) {
        self.editor.draw(cx);
    }

    fn children(&self) -> Vec<Token> {
        self.editor.tokens()
    }
}
