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
    Button(ElementPointer<PinWrapper<Button>>),
}

impl From<ElementPointer<PinWrapper<Todo>>> for AppPin {
    fn from(value: ElementPointer<PinWrapper<Todo>>) -> Self {
        AppPin::Todo(value)
    }
}

impl From<ElementPointer<PinWrapper<Button>>> for AppPin {
    fn from(value: ElementPointer<PinWrapper<Button>>) -> Self {
        AppPin::Button(value)
    }
}

impl Element for AppPin {
    fn update(&mut self, cx: &mut UpdateContext) {
        match self {
            AppPin::Todo(todo) => todo.update(cx),
            AppPin::Button(button) => button.update(cx),
        }
    }

    fn layout(&mut self, min: Size, max: Size, cx: &mut LayoutContext) -> Size {
        match self {
            AppPin::Todo(todo) => todo.layout(min, max, cx).position(Point::ZERO, cx),
            AppPin::Button(button) => button.layout(min, max, cx).position(Point::ZERO, cx),
        }
    }

    fn draw(&self, cx: &mut DrawContext) {
        match self {
            AppPin::Todo(todo) => todo.draw(cx),
            AppPin::Button(button) => button.draw(cx),
        }
    }
}

impl Pinnable for AppPin {
    fn center(&self) -> Point {
        match self {
            AppPin::Todo(todo) => todo.center(),
            AppPin::Button(button) => button.center(),
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
            .add_child(AppPin::from(Todo::new(Point::new(100., 100.))));
        board.borrow_mut().add_child(AppPin::from(PinWrapper::new(
            Point::new(200., 200.),
            Button::new(
                Size::new(100., 100.),
                *BACKGROUND1,
                *BACKGROUND2,
                |_rect, _cx| {},
                {
                    let board = board.clone();
                    move |cx| {
                        let mut board = board.borrow_mut();
                        board.add_child(AppPin::from(Todo::new(Point::new(300., 300.))));
                        cx.request_redraw();
                    }
                },
            ),
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
            .position(Point::ZERO, cx);
        self.window_buttons
            .layout(Size::new(0., 0.), max, cx)
            .position(Point::ZERO, cx);
        self.resize_handles
            .layout(min, max, cx)
            .position(Point::ZERO, cx);

        max
    }

    fn draw(&self, cx: &mut DrawContext) {
        self.board.borrow_mut().draw(cx);
        self.window_buttons.draw(cx);
        self.resize_handles.draw(cx);
    }
}
