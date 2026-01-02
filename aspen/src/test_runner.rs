use std::{collections::{HashMap, HashSet}, ops::Deref};

use mockall::predicate::eq;
use vello::{kurbo::Size, Scene};
use winit::{event::ElementState, keyboard::{Key, SmolStr}, window::Cursor};

use crate::{
    application::Application, context_stack::{
        KeyEvent, attached_context::{MockContextEventLoop, MockContextWindow}, context::Context
    }, element::{Element, ElementPointer}, token::Token
};


pub struct TestRunner<Root: Element> {
    pub application: Application<Root>,
    pub window: MockContextWindow,
    pub event_loop: MockContextEventLoop,
}

impl<Root: Element> TestRunner<Root> {
    pub fn new<F>(window_size: Size, root_constructor: F) -> Self 
        where F: for<'a> FnOnce(&mut Context<'a>) -> ElementPointer<Root>,
    {
        let mut application = Application::<Root>::new(root_constructor);
        application.event_state.window_size = window_size;

        let mut window = MockContextWindow::new();
        window.expect_request_redraw().return_const(());

        let event_loop = MockContextEventLoop::new();
        TestRunner::<Root> {
            application,
            window,
            event_loop,
        }
    }

    pub fn tick(&mut self) -> Option<Scene> {
        let mut drawn_scene = None;
        while let Some(newly_drawn_scene) = self.application.tick(&self.window, &self.event_loop) {
            // Continue to tick until draw not requested
            drawn_scene = Some(newly_drawn_scene);
        }
        drawn_scene
    }

    pub fn refresh_tokens(&mut self) {
        self.application.refresh_tokens();
    }

    pub fn process_mouse_regions(&self) -> bool {
        self.application.process_mouse_regions(&self.window, &self.event_loop)
    }

    pub fn update(&self) -> bool {
        self.application.update(&self.window, &self.event_loop)
    }

    pub fn layout(&self) -> HashMap<Token, HashSet<Token>> {
        self.application.layout(&self.window, &self.event_loop)
    }

    pub fn draw(&mut self, child_lookup: HashMap<Token, HashSet<Token>>) -> Scene {
        self.application.draw(child_lookup, &self.window, &self.event_loop)
    }

    /** Test Helpers */

    pub fn input_text(&mut self, text: &str) {
        for char in text.chars() {
            let str = SmolStr::new_inline(&char.to_string());
            self.application.event_state.key_events.push(KeyEvent {
                key: Key::Character(str.clone()),
                state: ElementState::Pressed,
            });
            self.application.event_state.key_events.push(KeyEvent {
                key: Key::Character(str.clone()),
                state: ElementState::Released,
            });
        }
        self.tick();
    }

    pub fn input_key(&mut self, key: Key) {
        self.application.event_state.key_events.push(KeyEvent {
            key: key.clone(),
            state: ElementState::Pressed,
        });
        self.application.event_state.key_events.push(KeyEvent {
            key,
            state: ElementState::Released,
        });
        self.tick();
    }

    pub fn with_root<Result>(&self, callback: impl FnOnce(&Root, &Context) -> Result) -> Result {
        self.application.with_root(callback)
    }

    pub fn expect_cursor_icon(&mut self, icon: Cursor) {
        self.window.expect_set_cursor()
            .with(eq(icon))
            .return_const(());
    }
}

impl<Root: Element> Deref for TestRunner<Root> {
    type Target = Application<Root>;

    fn deref(&self) -> &Self::Target {
        &self.application
    }
}
