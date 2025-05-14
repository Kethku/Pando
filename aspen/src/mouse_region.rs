use std::collections::HashSet;

use vello::kurbo::{Affine, BezPath, Point, Shape, Vec2};
use winit::window::CursorIcon;

use crate::{
    context::{Context, EventContext},
    token::Token,
};

const MIN_DRAG: f64 = 3.;

pub struct MouseRegion {
    token: Token,
    region: BezPath,
    transform: Affine,
    icon: Option<CursorIcon>,
    clip_stack: Vec<BezPath>,
    on_drag: Option<Box<dyn Fn(&mut EventContext)>>,
    on_right_drag: Option<Box<dyn Fn(&mut EventContext)>>,
    on_hover: Option<Box<dyn Fn(&mut EventContext)>>,
    on_leave: Option<Box<dyn Fn(&mut EventContext)>>,
    on_down: Option<Box<dyn Fn(&mut EventContext)>>,
    on_right_down: Option<Box<dyn Fn(&mut EventContext)>>,
    on_up: Option<Box<dyn Fn(&mut EventContext)>>,
    on_right_up: Option<Box<dyn Fn(&mut EventContext)>>,
    on_click: Option<Box<dyn Fn(&mut EventContext)>>,
    on_right_click: Option<Box<dyn Fn(&mut EventContext)>>,
    on_scroll: Option<Box<dyn Fn(&mut EventContext)>>,
}

impl MouseRegion {
    pub fn new(
        token: Token,
        shape: impl Shape,
        transform: Affine,
        clip_stack: Vec<BezPath>,
    ) -> Self {
        MouseRegion {
            region: transform * shape.to_path(0.1),
            token,
            transform,
            clip_stack,
            icon: None,
            on_drag: None,
            on_right_drag: None,
            on_hover: None,
            on_leave: None,
            on_down: None,
            on_right_down: None,
            on_up: None,
            on_right_up: None,
            on_click: None,
            on_right_click: None,
            on_scroll: None,
        }
    }

    fn contains(&self, point: Point) -> bool {
        self.region.contains(point)
    }

    pub fn with_icon(&mut self, icon: CursorIcon) -> &mut Self {
        self.icon = Some(icon);
        self
    }

    pub fn on_hover<F: Fn(&mut EventContext) + 'static>(&mut self, f: F) -> &mut Self {
        self.on_hover = Some(Box::new(f));
        self
    }

    pub fn on_leave<F: Fn(&mut EventContext) + 'static>(&mut self, f: F) -> &mut Self {
        self.on_leave = Some(Box::new(f));
        self
    }

    pub fn on_down<F: Fn(&mut EventContext) + 'static>(&mut self, f: F) -> &mut Self {
        self.on_down = Some(Box::new(f));
        self
    }

    pub fn on_drag<F: Fn(&mut EventContext) + 'static>(&mut self, f: F) -> &mut Self {
        self.on_drag = Some(Box::new(f));
        self
    }

    pub fn on_up<F: Fn(&mut EventContext) + 'static>(&mut self, f: F) -> &mut Self {
        self.on_up = Some(Box::new(f));
        self
    }

    pub fn on_click<F: Fn(&mut EventContext) + 'static>(&mut self, f: F) -> &mut Self {
        self.on_click = Some(Box::new(f));
        self
    }

    pub fn on_scroll<F: Fn(&mut EventContext) + 'static>(&mut self, f: F) -> &mut Self {
        self.on_scroll = Some(Box::new(f));
        self
    }
}

pub struct MouseRegionManager {
    down: Option<Point>,
    drag_min_reached: bool,
    current_dragger: Option<Token>,
    current_clicker: Option<Token>,
    regions: Vec<MouseRegion>,
    hovered_regions: HashSet<Token>,
}

impl MouseRegionManager {
    pub fn new() -> Self {
        MouseRegionManager {
            down: None,
            drag_min_reached: false,
            current_dragger: None,
            current_clicker: None,
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
        let mut icon_set = false;

        if cx.mouse_just_down() {
            self.down = Some(cx.actual_mouse_position());
            self.drag_min_reached = false;
        }

        let down = self.down;
        let mut drag_min_just_reached = false;
        if let Some(down) = down {
            if (cx.actual_mouse_position() - down).length() > MIN_DRAG {
                if !self.drag_min_reached {
                    drag_min_just_reached = true;
                }
                self.drag_min_reached = true;
                self.current_clicker = None;
            }
        }

        let current_dragger = self.current_dragger;
        let current_clicker = self.current_clicker;
        if !cx.mouse_down() {
            self.down = None;
            self.current_dragger = None;
            self.current_clicker = None;
        }

        let mut redraw_requested = false;
        let mut cx = EventContext::new(cx, &mut redraw_requested);
        for region in self.regions.iter().rev() {
            if self.hovered_regions.contains(&region.token) {
                if !region.contains(cx.actual_mouse_position()) {
                    if let Some(on_leave) = &region.on_leave {
                        cx.transform = region.transform;
                        on_leave(&mut cx);
                    }
                }
            }
        }

        // Left mouse button handling
        for region in self.regions.iter().rev() {
            for clip in region.clip_stack.iter() {
                if !clip.contains(cx.actual_mouse_position()) {
                    continue;
                }
            }

            let mut consume = false;

            if let Some(down) = down {
                if (cx.actual_mouse_position() - down).length() > MIN_DRAG {
                    self.drag_min_reached = true;
                }

                if current_dragger == Some(region.token) && cx.mouse_down() && self.drag_min_reached
                {
                    if let Some(on_drag) = &region.on_drag {
                        cx.transform = region.transform;
                        let down = cx.transform.inverse() * down;
                        if drag_min_just_reached {
                            cx.delta_correction = Some(cx.mouse_position() - down);
                        }
                        on_drag(&mut cx);
                        cx.delta_correction = None;
                        consume = true;
                    }
                }
            }

            if region.contains(cx.actual_mouse_position()) {
                if !icon_set {
                    if let Some(icon) = region.icon {
                        cx.transform = region.transform;
                        cx.set_cursor(icon);
                        icon_set = true;
                    }
                }

                self.hovered_regions.insert(region.token);

                if !cx.mouse_down() {
                    if let Some(on_hover) = &region.on_hover {
                        cx.transform = region.transform;
                        on_hover(&mut cx);
                        self.hovered_regions.insert(region.token);
                    }
                }

                if cx.mouse_just_down() {
                    if let Some(on_down) = &region.on_down {
                        cx.transform = region.transform;
                        on_down(&mut cx);
                    }

                    if self.current_dragger.is_none() && region.on_drag.is_some() {
                        self.current_dragger = Some(region.token);
                    }

                    if self.current_clicker.is_none() && region.on_click.is_some() {
                        self.current_clicker = Some(region.token);
                    }
                }

                if cx.mouse_released() {
                    if let Some(on_up) = &region.on_up {
                        cx.transform = region.transform;
                        on_up(&mut cx);
                    }

                    if let Some(on_click) = &region.on_click {
                        if !self.drag_min_reached {
                            if current_clicker == Some(region.token) {
                                cx.transform = region.transform;
                                on_click(&mut cx);
                                consume = true;
                            }
                        }
                    }
                }
            }

            if consume {
                break;
            }
        }

        for region in self.regions.iter().rev() {
            if region.contains(cx.actual_mouse_position()) {
                if cx.scroll_delta() != Vec2::ZERO {
                    if let Some(on_scroll) = &region.on_scroll {
                        cx.transform = region.transform;
                        on_scroll(&mut cx);
                        break;
                    }
                }
            }
        }

        if !icon_set {
            cx.set_cursor(CursorIcon::Default);
        }

        redraw_requested
    }
}
