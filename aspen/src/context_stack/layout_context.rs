use std::{
    any::Any,
    collections::{HashMap, HashSet, hash_map::Entry},
    ops::{Deref, DerefMut},
};

use vello::kurbo::{Affine, Size};

use super::AttachedContext;

use crate::token::Token;

pub struct LayoutContext<'a> {
    context: AttachedContext<'a>,
    regions: &'a mut HashMap<Token, (Affine, Size)>,
    children: &'a mut HashMap<Token, HashSet<Token>>,
}

impl<'a> Deref for LayoutContext<'a> {
    type Target = AttachedContext<'a>;

    fn deref(&self) -> &Self::Target {
        &self.context
    }
}

impl<'a> DerefMut for LayoutContext<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.context
    }
}

impl<'a> LayoutContext<'a> {
    pub fn new(
        context: AttachedContext<'a>,
        regions: &'a mut HashMap<Token, (Affine, Size)>,
        children: &'a mut HashMap<Token, HashSet<Token>>,
    ) -> LayoutContext<'a> {
        LayoutContext {
            context,
            regions,
            children,
        }
    }

    pub fn add_region(&mut self, element_token: Token, transform: Affine, size: Size) {
        self.children
            .entry(self.token())
            .or_default()
            .insert(element_token);
        self.regions.insert(element_token, (transform, size));
    }

    pub fn with_initialized_state<State: Any, Result>(&mut self, callback: impl FnOnce(&mut State, &mut LayoutContext<'a>) -> Result) -> Result {
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
        &mut self,
        callback: impl FnOnce(&mut State, &mut LayoutContext<'a>) -> Result,
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

    pub fn child<'b>(
        &'b mut self,
        element_token: Token,
        element_children: &'b Vec<Token>,
    ) -> LayoutContext<'b>
    where
        'a: 'b,
    {
        let child_cx: AttachedContext<'b> = self.context.child(element_token, element_children);
        LayoutContext::<'b> {
            context: child_cx,
            regions: self.regions,
            children: self.children,
        }
    }
}
