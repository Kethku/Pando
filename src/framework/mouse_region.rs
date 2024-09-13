use std::collections::HashSet;

use glamour::prelude::*;

use crate::framework::{context::Context, token::Token};

pub struct MouseRegion {
    rect: Rect,
    token: Token,
    on_drag: Option<Box<dyn Fn(Point2, &Context)>>,
    on_hover: Option<Box<dyn Fn(&Context)>>,
    on_leave: Option<Box<dyn Fn(&Context)>>,
    on_down: Option<Box<dyn Fn(&Context)>>,
    on_up: Option<Box<dyn Fn(&Context)>>,
    on_clicked: Option<Box<dyn Fn(&Context)>>,
}

impl MouseRegion {
    pub fn new(token: Token, rect: Rect) -> Self {
        MouseRegion {
            rect,
            token,
            on_hover: None,
            on_leave: None,
            on_down: None,
            on_drag: None,
            on_up: None,
            on_clicked: None,
        }
    }

    pub fn on_hover<F: Fn(&Context) + 'static>(mut self, f: F) -> Self {
        self.on_hover = Some(Box::new(f));
        self
    }

    pub fn on_leave<F: Fn(&Context) + 'static>(mut self, f: F) -> Self {
        self.on_leave = Some(Box::new(f));
        self
    }

    pub fn on_down<F: Fn(&Context) + 'static>(mut self, f: F) -> Self {
        self.on_down = Some(Box::new(f));
        self
    }

    pub fn on_drag<F: Fn(Point2, &Context) + 'static>(mut self, f: F) -> Self {
        self.on_drag = Some(Box::new(f));
        self
    }

    pub fn on_up<F: Fn(&Context) + 'static>(mut self, f: F) -> Self {
        self.on_up = Some(Box::new(f));
        self
    }

    pub fn on_clicked<F: Fn(&Context) + 'static>(mut self, f: F) -> Self {
        self.on_clicked = Some(Box::new(f));
        self
    }
}

pub struct MouseRegionManager {
    down: Option<Point2>,
    current_dragger: Option<Token>,
    regions: Vec<MouseRegion>,
    hovered_regions: HashSet<Token>,
}

impl MouseRegionManager {
    pub fn new() -> Self {
        MouseRegionManager {
            down: None,
            current_dragger: None,
            regions: Vec::new(),
            hovered_regions: HashSet::new(),
        }
    }

    pub fn clear_regions(&mut self) {
        self.regions.clear();
    }

    pub fn add_region(&mut self, region: MouseRegion) {
        self.regions.push(region);
    }

    pub fn process_regions(&mut self, cx: &Context) {
        let down = self.down;
        if cx.mouse_just_down() {
            self.down = Some(cx.mouse_position());
        }

        let current_dragger = self.current_dragger;
        if !cx.mouse_down() {
            self.down = None;
            self.current_dragger = None;
        }

        for region in self.regions.iter().rev() {
            if self.hovered_regions.contains(&region.token) {
                if !region.rect.contains(&cx.mouse_position()) {
                    if let Some(on_leave) = &region.on_leave {
                        on_leave(cx);
                    }
                }
            }
        }

        for region in self.regions.iter().rev() {
            if region.rect.contains(&cx.mouse_position()) {
                self.hovered_regions.insert(region.token);

                let mut consume = false;

                if !cx.mouse_down() {
                    if let Some(on_hover) = &region.on_hover {
                        on_hover(cx);
                        self.hovered_regions.insert(region.token);
                    }
                }

                if cx.mouse_just_down() {
                    if let Some(on_down) = &region.on_down {
                        on_down(cx);
                    }

                    self.current_dragger = Some(region.token);
                    consume = true;
                }

                if cx.mouse_released() {
                    if let Some(on_up) = &region.on_up {
                        on_up(cx);
                    }

                    if let Some(on_clicked) = &region.on_clicked {
                        if current_dragger == Some(region.token) {
                            on_clicked(cx);
                        }
                        consume = true;
                    }
                }

                if let Some(down) = down {
                    if let Some(on_drag) = &region.on_drag {
                        if current_dragger == Some(region.token) {
                            on_drag(down, cx);
                        }
                    }
                }

                if consume {
                    break;
                }
            }
        }
    }
}
