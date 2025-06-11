use std::ops::Deref;

use super::AttachedContext;

use crate::{
    mouse_region::{MouseRegion, MouseRegionManager},
    token::Token,
};

pub struct UpdateContext<'a> {
    context: AttachedContext<'a>,
    mouse_region_manager: &'a mut MouseRegionManager,
    redraw_requested: &'a mut bool,
}

impl<'a> Deref for UpdateContext<'a> {
    type Target = AttachedContext<'a>;

    fn deref(&self) -> &Self::Target {
        &self.context
    }
}

impl<'a> UpdateContext<'a> {
    pub fn new(
        context: AttachedContext<'a>,
        mouse_region_manager: &'a mut MouseRegionManager,
        redraw_requested: &'a mut bool,
    ) -> UpdateContext<'a> {
        UpdateContext {
            context,
            mouse_region_manager,
            redraw_requested,
        }
    }

    pub fn add_mouse_region(&mut self, mouse_region: MouseRegion) {
        self.mouse_region_manager.add_region(mouse_region);
    }

    pub fn request_redraw(&mut self) {
        *self.redraw_requested = true;
    }

    pub fn child<'b>(
        &'b mut self,
        element_token: Token,
        element_children: Vec<Token>,
    ) -> UpdateContext<'b>
    where
        'a: 'b,
    {
        let child_cx: AttachedContext<'b> = self.context.child(element_token, element_children);
        UpdateContext {
            context: child_cx,
            mouse_region_manager: self.mouse_region_manager,
            redraw_requested: self.redraw_requested,
        }
    }
}
