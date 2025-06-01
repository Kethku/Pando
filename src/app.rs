use std::{cell::RefCell, rc::Rc};

use aspen::prelude::*;

use crate::{todo::Todo, util::*};

pub struct App {
    pub board: Rc<RefCell<ElementPointer<Board<AppPin>>>>,

    pub window_buttons: ElementPointer<WindowButtons>,
    pub resize_handles: ElementPointer<ResizeHandles>,
}

pub enum AppPin {
    Todo(ElementPointer<PinWrapper<Todo>>),
    Board(ElementPointer<PinWrapper<Board<PinWrapper<Todo>>>>),
}

impl From<ElementPointer<PinWrapper<Todo>>> for AppPin {
    fn from(value: ElementPointer<PinWrapper<Todo>>) -> Self {
        AppPin::Todo(value)
    }
}

impl From<ElementPointer<PinWrapper<Board<PinWrapper<Todo>>>>> for AppPin {
    fn from(value: ElementPointer<PinWrapper<Board<PinWrapper<Todo>>>>) -> Self {
        AppPin::Board(value)
    }
}

impl Element for AppPin {
    fn update(&mut self, cx: &mut UpdateContext) {
        match self {
            AppPin::Todo(todo) => todo.update(cx),
            AppPin::Board(board) => board.update(cx),
        }
    }

    fn layout(&mut self, min: Size, max: Size, cx: &mut LayoutContext) -> Size {
        match self {
            AppPin::Todo(todo) => todo.layout(min, max, cx).position(Affine::IDENTITY, cx),
            AppPin::Board(board) => board.layout(min, max, cx).position(Affine::IDENTITY, cx),
        }
    }

    fn draw(&self, cx: &mut DrawContext) {
        match self {
            AppPin::Todo(todo) => todo.draw(cx),
            AppPin::Board(board) => board.draw(cx),
        }
    }
}

impl Pinnable for AppPin {
    fn center(&self) -> Point {
        match self {
            AppPin::Todo(todo) => todo.center(),
            AppPin::Board(board) => board.center(),
        }
    }
}

impl App {
    pub fn new() -> App {
        let board = Rc::new(RefCell::new(Board::new_dotgrid(
            Affine::IDENTITY,
            *BACKGROUND0,
            *BACKGROUND3,
        )));
        board
            .borrow_mut()
            .add_child(AppPin::from(Todo::new(Point::new(-100., -100.))));
        board
            .borrow_mut()
            .add_child(AppPin::from(PinWrapper::new_sized(
                Point::new(000., 200.),
                Size::new(300., 300.),
                {
                    let mut board =
                        Board::new_dotgrid(Affine::IDENTITY, *BACKGROUND1, *BACKGROUND4);
                    board.add_child(Todo::new(Point::ZERO));
                    board
                },
            )));

        App {
            window_buttons: WindowButtons::new(*BACKGROUND3, *CLOSE, *BACKGROUND4, *FOREGROUND),
            resize_handles: ResizeHandles::new(),
            board,
        }
    }
}

impl Element for App {
    fn update(&mut self, cx: &mut UpdateContext) {
        self.board.borrow_mut().update(cx);
        self.window_buttons.update(cx);
        self.resize_handles.update(cx);
    }

    fn layout(&mut self, min: Size, max: Size, cx: &mut LayoutContext) -> Size {
        self.board
            .borrow_mut()
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
        self.board.borrow_mut().draw(cx);
        self.window_buttons.draw(cx);
        self.resize_handles.draw(cx);
    }
}
