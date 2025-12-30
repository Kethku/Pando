use std::{
    any::Any,
    collections::{hash_map::Entry, HashMap},
    ops::Deref,
};

use vello::kurbo::{Affine, Point, Rect, Size, Vec2};

use super::AttachedContext;

use crate::{
    element::{Element, ElementPointer},
    prelude::MouseRegion,
    token::Token,
};

/** Context specialized for mouse region events. */
pub struct EventContext<'a> {
    context: AttachedContext<'a>,
    redraw_requested: &'a mut bool,
    regions: &'a HashMap<Token, (Affine, Size)>,
    // Used when a drag just crossed the min threshold to report as a drag so that the dragger can
    // get a delta value that includes the threshold distance for the first mouse delta.
    //
    // Note: this presents some slight weirdness because mouse_delta will be larger than the actual
    // computed delta but so be it.
    pub(crate) delta_correction: Option<Vec2>,
    pub(crate) transform: Affine,
}

impl<'a> Deref for EventContext<'a> {
    type Target = AttachedContext<'a>;

    fn deref(&self) -> &Self::Target {
        &self.context
    }
}

impl<'a> EventContext<'a> {
    pub fn new(
        context: AttachedContext<'a>,
        redraw_requested: &'a mut bool,
        regions: &'a HashMap<Token, (Affine, Size)>,
    ) -> EventContext<'a> {
        EventContext {
            context,
            redraw_requested,
            regions,
            delta_correction: None,
            transform: Affine::IDENTITY,
        }
    }

    pub fn request_redraw(&mut self) {
        *self.redraw_requested = true;
    }

    /** Returns the mouse position relative to this context's current transform */
    pub fn mouse_position(&self) -> Option<Point> {
        self.actual_mouse_position()
            .map(|pos| self.transform.inverse() * pos)
    }

    pub fn mouse_position_relative_to<Other: Element>(
        &self,
        other: &ElementPointer<Other>,
    ) -> Option<Point> {
        self.regions
            .get(&other.token())
            .map(|(transform, _)| {
                self.actual_mouse_position()
                    .map(|pos| transform.inverse() * pos)
            })
            .expect(&format!(
                "Layout must not have been completed for this element before drawing: {:?}",
                other.token()
            ))
    }

    pub fn previous_mouse_position(&self) -> Option<Point> {
        self.actual_previous_mouse_position()
            .map(|pos| self.transform.inverse() * pos)
    }

    pub fn previous_mouse_position_relative_to<Other: Element>(
        &self,
        other: &ElementPointer<Other>,
    ) -> Option<Point> {
        self.regions
            .get(&other.token())
            .map(|(transform, _)| {
                self.actual_previous_mouse_position()
                    .map(|pos| transform.inverse() * pos)
            })
            .expect(&format!(
                "Layout must not have been completed for this element before drawing: {:?}",
                other.token()
            ))
    }

    pub fn mouse_delta(&self) -> Option<Vec2> {
        if let Some(delta) = self.delta_correction {
            Some(delta)
        } else {
            self.mouse_position()
                .zip(self.previous_mouse_position())
                .map(|(pos, prev)| pos - prev)
        }
    }

    pub fn mouse_delta_relative_to<Other: Element>(
        &self,
        other: &ElementPointer<Other>,
    ) -> Option<Vec2> {
        self.regions
            .get(&other.token())
            .map(|(transform, _)| {
                self.actual_mouse_position()
                    .zip(self.actual_previous_mouse_position())
                    .map(|(pos, prev)| {
                        let inverse = transform.inverse();
                        inverse * pos - inverse * prev
                    })
            })
            .expect(&format!(
                "Layout must not have been completed for this element before drawing: {:?}",
                other.token()
            ))
    }

    pub fn window_bounding_box(&self) -> Rect {
        self.transform
            .inverse()
            .transform_rect_bbox(self.actual_window_rect())
    }

    pub fn focus(&mut self) {
        self.context.focus();
        self.request_redraw();
    }

    pub fn with_initialized_state<State: Any, Result>(
        &mut self,
        callback: impl FnOnce(&mut State, &mut EventContext<'a>) -> Result,
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
        &mut self,
        callback: impl FnOnce(&mut State, &mut EventContext<'a>) -> Result,
    ) -> Result {
        let mut states = self.states.borrow_mut();
        let state = match states.entry(self.element_token) {
            Entry::Occupied(entry) => entry
                .into_mut()
                .downcast_mut()
                .expect("Tried to get state with different type than was previously inserted"),
            Entry::Vacant(entry) => entry
                .insert(Box::new(State::default()))
                .downcast_mut()
                .unwrap(),
        };

        callback(state, self)
    }

    pub fn for_region<'b>(&'b mut self, region: &'b MouseRegion) -> EventContext<'b> {
        let child_cx: AttachedContext<'b> = self
            .context
            .child(region.token.token, &region.element_children);

        EventContext {
            context: child_cx,

            redraw_requested: self.redraw_requested,
            regions: self.regions,
            delta_correction: self.delta_correction,
            transform: region.transform,
        }
    }
}
