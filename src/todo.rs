use std::{cell::RefCell, rc::Rc};

use aspen::prelude::*;

use crate::util::*;

pub struct Todo {
    editor: ElementPointer<TextEditor>,

    state: Rc<RefCell<TodoState>>,
}

struct TodoState {
    center: Point2,
}

impl Todo {
    pub fn new(center: Point2) -> ElementPointer<Self> {
        ElementPointer::new(Self {
            editor: TextEditor::new(*FOREGROUND),

            state: Rc::new(RefCell::new(TodoState { center })),
        })
    }
}

impl Element for Todo {
    fn layout(&mut self, min: Size2, max: Size2, cx: &mut LayoutContext) -> Size2 {
        let editor_result = self
            .editor
            .layout(min - size2!(20., 20.), max - size2!(20., 20.), cx);
        let todo_size = editor_result.size() + size2!(20., 20.);
        editor_result.position(point2!(10., 10.), cx);

        todo_size.clamp(min, max)
    }

    fn draw(&self, cx: &mut DrawContext) {
        let region = cx.region();

        cx.add_mouse_region(MouseRegion::new(cx.token(), region).on_drag({
            let state = self.state.clone();
            move |_down, cx| {
                let mut state = state.borrow_mut();
                state.center += cx.mouse_delta();
                cx.request_redraw();
            }
        }));

        cx.add_layer(
            Layer::new()
                .with_quad(
                    Quad::new(
                        region.translate(vector!(0., 2.5)),
                        Srgba::new(0., 0., 0., 0.6),
                    )
                    .with_edge_blur(10.)
                    .with_corner_radius(10.),
                )
                .with_quad(
                    Quad::new(region.inflate((2., 2.).into()), *BACKGROUND5).with_corner_radius(7.),
                )
                .with_quad(Quad::new(region, *BACKGROUND1).with_corner_radius(5.)),
        );

        self.editor.draw(cx);
    }
}

impl Pinnable for Todo {
    fn center(&self) -> Point2 {
        self.state.borrow().center
    }
}
