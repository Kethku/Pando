use std::{cell::RefCell, rc::Rc};

use aspen::prelude::*;

use crate::util::*;

pub struct Todo {
    editor: ElementPointer<TextEditor>,

    state: Rc<RefCell<TodoState>>,
}

struct TodoState {
    center: Point,
}

impl Todo {
    pub fn new(center: Point) -> ElementPointer<Self> {
        ElementPointer::new(Self {
            editor: TextEditor::new(Brush::Solid(*FOREGROUND)),

            state: Rc::new(RefCell::new(TodoState { center })),
        })
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
        let region = cx.region().inflate(2., 2.).to_rounded_rect(5.);

        cx.mouse_region(region).on_drag({
            let state = self.state.clone();
            move |_down, cx| {
                let mut state = state.borrow_mut();
                state.center += cx.mouse_delta();
                cx.request_redraw();
            }
        });
        cx.set_fill_brush(Brush::Solid(Color::new([0., 0., 0., 0.6])));
        cx.blurred(region + Vec2::new(0., 2.5), 10.);
        cx.set_fill_brush(Brush::Solid(*BACKGROUND1));
        cx.set_stroke_brush(Brush::Solid(*BACKGROUND5));
        cx.set_stroke_style(Stroke::new(2.));
        cx.stroked_fill(&region);

        self.editor.draw(cx);
    }
}

impl Pinnable for Todo {
    fn center(&self) -> Point {
        self.state.borrow().center
    }
}
