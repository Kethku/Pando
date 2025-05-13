use aspen::prelude::*;

use crate::util::*;

pub struct Todo {
    editor: ElementPointer<TextEditor>,
}

impl Todo {
    pub fn new(center: Point) -> ElementPointer<PinWrapper<Self>> {
        PinWrapper::new(
            center,
            ElementPointer::new(Self {
                editor: TextEditor::new(Brush::Solid(*FOREGROUND)),
            }),
        )
    }
}

impl Element for Todo {
    fn layout(&mut self, min: Size, max: Size, cx: &mut LayoutContext) -> Size {
        let editor_result =
            self.editor
                .layout(min - Size::new(20., 20.), max - Size::new(20., 20.), cx);
        let todo_size = editor_result.size() + Size::new(20., 20.);
        editor_result.position(Point::new(10., 10.), cx);

        todo_size.clamp(min, max)
    }

    fn draw(&self, cx: &mut DrawContext) {
        let region = cx.region().to_rounded_rect(5.);
        cx.set_fill_brush(Brush::Solid(Color::new([0., 0., 0., 0.6])));
        cx.blurred(region + Vec2::new(0., 2.5), 10.);
        cx.set_fill_brush(Brush::Solid(*BACKGROUND1));
        cx.set_stroke_brush(Brush::Solid(*BACKGROUND5));
        cx.set_stroke_style(Stroke::new(2.));
        cx.stroked_fill(&region);

        self.editor.draw(cx);
    }
}
