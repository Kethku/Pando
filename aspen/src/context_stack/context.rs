use std::{any::Any, cell::RefCell, collections::{HashMap, hash_map::Entry}, default::Default, ops::Deref};

use parley::{style::StyleProperty, Layout};
use vello::peniko::Brush;

use super::EventState;

use crate::{shaper::Shaper, token::Token};

pub struct Context<'a> {
    event_state: &'a EventState,
    pub(crate) shaper: &'a RefCell<Shaper>,
    default_text_styles: Vec<StyleProperty<'static, Brush>>,
    pub(crate) states: &'a RefCell<HashMap<Token, Box<dyn Any>>>,
    focused_element: &'a RefCell<Option<Token>>,
    pub(crate) element_token: Token,
    element_children: &'a Vec<Token>,
}

impl<'a> Deref for Context<'a> {
    type Target = EventState;

    fn deref(&self) -> &Self::Target { self.event_state
    }
}

impl<'a> Context<'a> {
    pub fn new(
        event_state: &'a EventState,
        shaper: &'a RefCell<Shaper>,
        states: &'a RefCell<HashMap<Token, Box<dyn Any>>>,
        focused_element: &'a RefCell<Option<Token>>,
        element_token: Token,
        element_children: &'a Vec<Token>,
    ) -> Context<'a> {
        Context {
            event_state,
            shaper,
            default_text_styles: Vec::new(),
            states,
            focused_element,
            element_token,
            element_children,
        }
    }

    pub fn push_default_text_style(&mut self, style: StyleProperty<'static, Brush>) {
        self.default_text_styles.push(style);
    }

    pub fn clear_default_text_styles(&mut self) {
        self.default_text_styles.clear();
    }

    pub fn layout(&self, text: &str) -> Layout<Brush> {
        self.shaper
            .borrow_mut()
            .layout(text, &self.default_text_styles)
    }

    pub fn layout_within(&self, text: &str, max_advance: f32) -> Layout<Brush> {
        self.shaper
            .borrow_mut()
            .layout_within(text, max_advance, &self.default_text_styles)
    }

    pub fn insert_state<State: Any>(&self, state: State) {
        let mut states = self.states.borrow_mut();
        states.insert(self.element_token, Box::new(state));
    }

    pub fn with_initialized_state<State: Any, Result>(
        &self,
        callback: impl FnOnce(&mut State, &Self) -> Result,
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

        callback(state, self)
    }

    pub fn with_state<State: Any + Default, Result>(
        &self,
        callback: impl FnOnce(&mut State, &Self) -> Result,
    ) -> Result {
        let mut states = self.states.borrow_mut();
        let state = match states.entry(self.element_token) {
            Entry::Occupied(entry) => {
                entry.into_mut().downcast_mut().expect("Tried to get state with different type than was previously inserted")
            },
            Entry::Vacant(entry) => {
                entry.insert(Box::new(State::default())).downcast_mut().unwrap()
            }
        };

        callback(state, self)
    }

    pub fn is_directly_focused(&self) -> bool {
        self.focused_element
            .borrow()
            .as_ref()
            .map_or(false, |token| token == &self.element_token)
    }

    pub fn is_focused(&self) -> bool {
        self.focused_element
            .borrow()
            .as_ref()
            .map_or(false, |token| self.tokens().contains(token))
    }

    pub fn focus(&self) {
        let mut focused_element = self.focused_element.borrow_mut();
        *focused_element = Some(self.token());
    }

    pub fn token(&self) -> Token {
        self.element_token
    }

    pub fn tokens(&self) -> Vec<Token> {
        let mut tokens = self.element_children.clone();
        tokens.push(self.element_token);
        tokens
    }

    pub(crate) fn children(&self) -> &Vec<Token> {
        &self.element_children
    }

    pub fn child<'b>(&self, element_token: Token, element_children: &'a Vec<Token>) -> Context<'b>
    where
        'a: 'b,
    {
        Context {
            event_state: self.event_state,
            shaper: self.shaper,
            states: self.states,
            default_text_styles: self.default_text_styles.clone(),
            focused_element: self.focused_element,
            element_token,
            element_children,
        }
    }
}
