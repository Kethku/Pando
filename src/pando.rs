use aspen::prelude::*;

use crate::{todo::Todo, util::*};

pub struct Pando {
    board: ElementPointer<Board>,
    window_buttons: ElementPointer<WindowButtons>,
    resize_handles: ElementPointer<ResizeHandles>,
}

impl Pando {
    pub fn new(cx: &mut Context) -> ElementPointer<Pando> {
        let mut board = Board::new_dotgrid(Affine::IDENTITY, *BACKGROUND0, *BACKGROUND3, cx);
        board.add_child(Todo::new(cx).as_pinnable(Point::new(-100., -100.), cx));
        board.add_child(PinWrapper::new_sized(
            Point::new(000., 200.),
            Size::new(300., 300.),
            {
                let mut board =
                    Board::new_dotgrid(Affine::IDENTITY, *BACKGROUND1, *BACKGROUND4, cx);
                board.add_child(Todo::new(cx).as_pinnable(Point::ZERO, cx));
                board
            },
            cx,
        ));

        ElementPointer::new(Pando {
            window_buttons: WindowButtons::new(*BACKGROUND3, *CLOSE, *BACKGROUND4, *FOREGROUND),
            resize_handles: ResizeHandles::new(),
            board,
        })
    }
}

impl Element for Pando {
    fn update(&mut self, cx: &mut UpdateContext) {
        self.board.update(cx);
        self.window_buttons.update(cx);
        self.resize_handles.update(cx);
    }

    fn layout(&mut self, min: Size, max: Size, cx: &mut LayoutContext) -> Size {
        self.board
            .layout(min, max, cx)
            .position(Affine::IDENTITY, cx);
        self.window_buttons
            .layout(Size::new(0., 0.), max, cx)
            .position(Affine::IDENTITY, cx);
        self.resize_handles
            .layout(min, max, cx)
            .position(Affine::IDENTITY, cx);

        max.max(Size::new(100., 100.))
    }

    fn draw(&self, cx: &mut DrawContext) {
        self.board.draw(cx);
        self.window_buttons.draw(cx);
        self.resize_handles.draw(cx);
    }

    fn children(&self) -> Vec<Token> {
        vec![
            self.board.tokens(),
            self.window_buttons.tokens(),
            self.resize_handles.tokens(),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}
