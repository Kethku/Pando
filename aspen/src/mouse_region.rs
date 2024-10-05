use std::collections::HashSet;

use vide::{prelude::*, winit::window::CursorIcon};

use crate::{
    context::{Context, EventContext},
    token::Token,
};

pub struct MouseRegion {
    rect: Rect,
    token: Token,
    icon: Option<CursorIcon>,
    on_drag: Option<Box<dyn Fn(Point2, &mut EventContext)>>,
    on_hover: Option<Box<dyn Fn(&mut EventContext)>>,
    on_leave: Option<Box<dyn Fn(&mut EventContext)>>,
    on_down: Option<Box<dyn Fn(&mut EventContext)>>,
    on_up: Option<Box<dyn Fn(&mut EventContext)>>,
    on_clicked: Option<Box<dyn Fn(&mut EventContext)>>,
}

impl MouseRegion {
    pub fn new(token: Token, rect: Rect) -> Self {
        MouseRegion {
            rect,
            token,
            icon: None,
            on_hover: None,
            on_leave: None,
            on_down: None,
            on_drag: None,
            on_up: None,
            on_clicked: None,
        }
    }

    pub fn with_icon(mut self, icon: CursorIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn on_hover<F: Fn(&mut EventContext) + 'static>(mut self, f: F) -> Self {
        self.on_hover = Some(Box::new(f));
        self
    }

    pub fn on_leave<F: Fn(&mut EventContext) + 'static>(mut self, f: F) -> Self {
        self.on_leave = Some(Box::new(f));
        self
    }

    pub fn on_down<F: Fn(&mut EventContext) + 'static>(mut self, f: F) -> Self {
        self.on_down = Some(Box::new(f));
        self
    }

    pub fn on_drag<F: Fn(Point2, &mut EventContext) + 'static>(mut self, f: F) -> Self {
        self.on_drag = Some(Box::new(f));
        self
    }

    pub fn on_up<F: Fn(&mut EventContext) + 'static>(mut self, f: F) -> Self {
        self.on_up = Some(Box::new(f));
        self
    }

    pub fn on_clicked<F: Fn(&mut EventContext) + 'static>(mut self, f: F) -> Self {
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

    pub fn add_region(&mut self, region: MouseRegion) -> &mut MouseRegion {
        self.regions.push(region);
        self.regions.last_mut().unwrap()
    }

    pub fn process_regions(&mut self, cx: &Context) -> bool {
        let down = self.down;
        let mut icon_set = false;

        if cx.mouse_just_down() {
            self.down = Some(cx.mouse_position());
        }

        let current_dragger = self.current_dragger;
        if !cx.mouse_down() {
            self.down = None;
            self.current_dragger = None;
        }

        let mut redraw_requested = false;
        let mut cx = EventContext::new(cx, &mut redraw_requested);
        for region in self.regions.iter().rev() {
            if self.hovered_regions.contains(&region.token) {
                if !region.rect.contains(&cx.mouse_position()) {
                    if let Some(on_leave) = &region.on_leave {
                        on_leave(&mut cx);
                    }
                }
            }
        }

        for region in self.regions.iter().rev() {
            if let Some(down) = down {
                if current_dragger == Some(region.token) && cx.mouse_down() {
                    if let Some(on_drag) = &region.on_drag {
                        on_drag(down, &mut cx);
                    }
                }
            }

            if region.rect.contains(&cx.mouse_position()) {
                if !icon_set {
                    if let Some(icon) = region.icon {
                        cx.set_cursor(icon);
                        icon_set = true;
                    }
                }

                self.hovered_regions.insert(region.token);

                let mut consume = false;

                if !cx.mouse_down() {
                    if let Some(on_hover) = &region.on_hover {
                        on_hover(&mut cx);
                        self.hovered_regions.insert(region.token);
                    }
                }

                if cx.mouse_just_down() {
                    if let Some(on_down) = &region.on_down {
                        on_down(&mut cx);
                    }

                    self.current_dragger = Some(region.token);
                    consume = true;
                }

                if cx.mouse_released() {
                    if let Some(on_up) = &region.on_up {
                        on_up(&mut cx);
                    }

                    if let Some(on_clicked) = &region.on_clicked {
                        if current_dragger == Some(region.token) {
                            on_clicked(&mut cx);
                        }
                        consume = true;
                    }
                }

                if consume {
                    break;
                }
            }
        }

        if !icon_set {
            cx.set_cursor(CursorIcon::Default);
        }

        redraw_requested
    }
}
