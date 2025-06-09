use std::{
    collections::{HashMap, HashSet},
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
            .entry(*self.token())
            .or_default()
            .insert(element_token);
        self.regions.insert(element_token, (transform, size));
    }

    pub fn child<'b>(&'b mut self, token: Token) -> LayoutContext<'b>
    where
        'a: 'b,
    {
        let child_cx: AttachedContext<'b> = self.context.child(token);
        LayoutContext::<'b> {
            context: child_cx,
            regions: self.regions,
            children: self.children,
        }
    }
}
