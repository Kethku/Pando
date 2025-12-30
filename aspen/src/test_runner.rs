use std::{collections::{HashMap, HashSet}, ops::Deref, sync::Arc};

use vello::{kurbo::Size, Scene};

use crate::{
    application::Application, context_stack::{
        attached_context::{MockContextEventLoop, MockContextWindow},
        context::Context,
    }, element::{Element, ElementPointer}, token::Token
};


pub struct TestRunner<Root: Element> {
    pub application: Application<Root>,
    pub window: Arc<MockContextWindow>,
    pub event_loop: MockContextEventLoop,
}

impl<Root: Element> TestRunner<Root> {
    pub fn new<F>(window_size: Size, root_constructor: F) -> Self 
        where F: for<'a> FnOnce(&mut Context<'a>) -> ElementPointer<Root>,
    {
        let mut application = Application::<Root>::new(root_constructor);
        application.event_state.window_size = window_size;
        TestRunner::<Root> {
            application,
            window: Arc::new(MockContextWindow::new()),
            event_loop: MockContextEventLoop::new(),
        }
    }

    pub fn tick(&mut self) -> Option<Scene> {
        self.application.tick(self.window.clone(), &self.event_loop)
    }

    pub fn refresh_tokens(&mut self) {
        self.application.refresh_tokens();
    }

    pub fn process_mouse_regions(&self) -> bool {
        self.application.process_mouse_regions(self.window.clone(), &self.event_loop)
    }

    pub fn update(&self) -> bool {
        self.application.update(self.window.clone(), &self.event_loop)
    }

    pub fn layout(&self) -> HashMap<Token, HashSet<Token>> {
        self.application.layout(self.window.clone(), &self.event_loop)
    }

    pub fn draw(&mut self, child_lookup: HashMap<Token, HashSet<Token>>) -> Scene {
        self.application.draw(child_lookup, self.window.clone(), &self.event_loop)
    }
}

impl<Root: Element> Deref for TestRunner<Root> {
    type Target = Application<Root>;

    fn deref(&self) -> &Self::Target {
        &self.application
    }
}
