use std::{
    any::Any, cell::RefCell, collections::HashMap, marker::PhantomData, ops::Deref, sync::Arc,
};

use mockall::*;
use parley::{style::StyleProperty, Layout};
use vello::peniko::Brush;

use super::EventState;

use crate::{element::Element, shaper::Shaper, token::Token};

pub struct Context<'a> {
    event_state: &'a EventState,
    shaper: &'a RefCell<Shaper>,
    states: &'a RefCell<HashMap<Token, Box<dyn Any>>>,
    default_text_styles: Vec<StyleProperty<'static, Brush>>,
    element_token: Token,
}

impl<'a> Deref for Context<'a> {
    type Target = EventState;

    fn deref(&self) -> &Self::Target {
        self.event_state
    }
}

impl<'a> Context<'a> {
    pub fn new(
        event_state: &'a EventState,
        shaper: &'a RefCell<Shaper>,
        states: &'a RefCell<HashMap<Token, Box<dyn Any>>>,
        element_token: Token,
    ) -> Context<'a> {
        Context {
            event_state,
            shaper,
            states,
            default_text_styles: Vec::new(),
            element_token,
        }
    }

    pub fn push_default_text_style(&mut self, style: StyleProperty<'static, Brush>) {
        self.default_text_styles.push(style);
    }

    pub fn clear_default_text_styles(&mut self) {
        self.default_text_styles.clear();
    }

    pub fn layout(&mut self, text: &str) -> Layout<Brush> {
        self.shaper
            .borrow_mut()
            .layout(text, &self.default_text_styles)
    }

    pub fn layout_within(&mut self, text: &str, max_advance: f32) -> Layout<Brush> {
        self.shaper
            .borrow_mut()
            .layout_within(text, max_advance, &self.default_text_styles)
    }

    pub fn insert_state<State: Any>(&self, state: State) {
        let mut states = self.states.borrow_mut();
        states.insert(self.element_token, Box::new(state));
    }

    pub fn with_state<State: Any, Result>(
        &self,
        callback: impl FnOnce(&mut State) -> Result,
    ) -> Result {
        let mut states = self.states.borrow_mut();
        let state = states
            .get_mut(&self.element_token)
            .expect(&format!(
                "Tried to get state that hasn't be initialized for {:?}",
                &self.element_token
            ))
            .downcast_mut()
            .expect("Tried to get state with different type than previous fetch");

        callback(state)
    }

    pub fn token(&self) -> &Token {
        &self.element_token
    }

    pub fn child<'b>(&self, element_token: Token) -> Context<'b>
    where
        'a: 'b,
    {
        Context {
            event_state: self.event_state,
            shaper: self.shaper,
            states: self.states,
            default_text_styles: self.default_text_styles.clone(),
            element_token,
        }
    }
}
