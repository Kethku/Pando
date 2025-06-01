use std::collections::{HashMap, HashSet};

use vello::{
    kurbo::{Affine, BezPath, Point, Shape, Size, Vec2},
    peniko::{Brush, Color, Fill},
    Scene,
};
use winit::window::CursorIcon;

use crate::{
    context::{Context, EventContext},
    token::Token,
};

const MIN_DRAG: f64 = 3.;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RegionToken {
    pub(crate) token: Token,
    pub(crate) index: usize,
}

pub struct MouseRegion {
    token: RegionToken,
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
        token: RegionToken,
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

    pub fn on_drag<F: Fn(&mut EventContext) + 'static>(&mut self, f: F) -> &mut Self {
        self.on_drag = Some(Box::new(f));
        self
    }

    pub fn on_right_drag<F: Fn(&mut EventContext) + 'static>(&mut self, f: F) -> &mut Self {
        self.on_right_drag = Some(Box::new(f));
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

    pub fn on_right_down<F: Fn(&mut EventContext) + 'static>(&mut self, f: F) -> &mut Self {
        self.on_right_down = Some(Box::new(f));
        self
    }

    pub fn on_up<F: Fn(&mut EventContext) + 'static>(&mut self, f: F) -> &mut Self {
        self.on_up = Some(Box::new(f));
        self
    }

    pub fn on_right_up<F: Fn(&mut EventContext) + 'static>(&mut self, f: F) -> &mut Self {
        self.on_up = Some(Box::new(f));
        self
    }

    pub fn on_click<F: Fn(&mut EventContext) + 'static>(&mut self, f: F) -> &mut Self {
        self.on_click = Some(Box::new(f));
        self
    }

    pub fn on_right_click<F: Fn(&mut EventContext) + 'static>(&mut self, f: F) -> &mut Self {
        self.on_up = Some(Box::new(f));
        self
    }

    pub fn on_scroll<F: Fn(&mut EventContext) + 'static>(&mut self, f: F) -> &mut Self {
        self.on_scroll = Some(Box::new(f));
        self
    }
}

pub struct MouseRegionManager {
    pub(crate) mouse_regions: Vec<MouseRegion>,
    down: Option<Point>,
    right_down: Option<Point>,
    drag_min_reached: bool,
    right_drag_min_reached: bool,
    current_dragger: Option<RegionToken>,
    current_right_dragger: Option<RegionToken>,
    current_clicker: Option<RegionToken>,
    current_right_clicker: Option<RegionToken>,
    hovered_regions: HashSet<RegionToken>,
}

impl MouseRegionManager {
    pub fn new() -> Self {
        MouseRegionManager {
            mouse_regions: Vec::new(),
            down: None,
            right_down: None,
            drag_min_reached: false,
            right_drag_min_reached: false,
            current_dragger: None,
            current_right_dragger: None,
            current_clicker: None,
            current_right_clicker: None,
            hovered_regions: HashSet::new(),
        }
    }

    pub fn clear_regions(&mut self) {
        self.mouse_regions.clear();
    }

    pub fn add_region(&mut self, region: MouseRegion) -> &mut MouseRegion {
        self.mouse_regions.push(region);
        self.mouse_regions.last_mut().unwrap()
    }

    /// Indicates if any mouse regions associated with this Token are currently tracked. Useful to
    /// continue rendering even if a region would normally be culled.
    pub fn token_currently_tracked(&self, token: &Token) -> bool {
        self.current_dragger
            .map_or(false, |region_token| &region_token.token == token)
            || self
                .current_right_dragger
                .map_or(false, |region_token| &region_token.token == token)
            || self
                .current_clicker
                .map_or(false, |region_token| &region_token.token == token)
            || self
                .current_right_clicker
                .map_or(false, |region_token| &region_token.token == token)
    }

    pub fn process_regions(
        &mut self,
        regions: &mut HashMap<Token, (Affine, Size)>,
        cx: &Context,
    ) -> bool {
        let mut icon_set = false;

        if cx.mouse_just_down() {
            self.down = Some(cx.actual_mouse_position());
            self.drag_min_reached = false;
        }

        if cx.right_mouse_just_down() {
            self.right_down = Some(cx.actual_mouse_position());
            self.right_drag_min_reached = false;
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

        let right_down = self.right_down;
        let mut right_drag_min_just_reached = false;
        if let Some(right_down) = right_down {
            if (cx.actual_mouse_position() - right_down).length() > MIN_DRAG {
                if !self.right_drag_min_reached {
                    right_drag_min_just_reached = true;
                }
                self.right_drag_min_reached = true;
                self.current_right_clicker = None;
            }
        }

        let current_dragger = self.current_dragger;
        let current_clicker = self.current_clicker;
        if !cx.mouse_down() {
            self.down = None;
            self.current_dragger = None;
            self.current_clicker = None;
        }

        let current_right_dragger = self.current_right_dragger;
        let current_right_clicker = self.current_clicker;
        if !cx.right_mouse_down() {
            self.right_down = None;
            self.current_right_dragger = None;
            self.current_right_clicker = None;
        }

        let mut redraw_requested = false;
        let mut cx = EventContext::new(cx, &mut redraw_requested, regions);
        for region in self.mouse_regions.iter().rev() {
            if self.hovered_regions.contains(&region.token) {
                if !region.contains(cx.actual_mouse_position()) {
                    if let Some(on_leave) = &region.on_leave {
                        cx.transform = region.transform;
                        on_leave(&mut cx);
                    }
                }
            }
        }

        let mut left_consumed = false;
        let mut right_consumed = false;
        for region in self.mouse_regions.iter().rev() {
            let mut clipped = false;
            for clip in region.clip_stack.iter() {
                if !clip.contains(cx.actual_mouse_position()) {
                    clipped = true;
                }
            }

            if let Some(down) = down {
                if (cx.actual_mouse_position() - down).length() > MIN_DRAG {
                    self.drag_min_reached = true;
                }

                if current_dragger == Some(region.token)
                    && cx.mouse_down()
                    && self.drag_min_reached
                    && !left_consumed
                {
                    if let Some(on_drag) = &region.on_drag {
                        cx.transform = region.transform;
                        let down = cx.transform.inverse() * down;
                        if drag_min_just_reached {
                            cx.delta_correction = Some(cx.mouse_position() - down);
                        }
                        on_drag(&mut cx);
                        cx.delta_correction = None;
                        left_consumed = true;
                    }
                }
            }

            if let Some(right_down) = right_down {
                if (cx.actual_mouse_position() - right_down).length() > MIN_DRAG {
                    self.drag_min_reached = true;
                }

                if current_right_dragger == Some(region.token)
                    && cx.right_mouse_down()
                    && self.right_drag_min_reached
                    && !right_consumed
                {
                    if let Some(on_right_drag) = &region.on_right_drag {
                        cx.transform = region.transform;
                        let right_down = cx.transform.inverse() * right_down;
                        if right_drag_min_just_reached {
                            cx.delta_correction = Some(cx.mouse_position() - right_down);
                        }
                        on_right_drag(&mut cx);
                        cx.delta_correction = None;
                        right_consumed = true;
                    }
                }
            }

            if region.contains(cx.actual_mouse_position()) && !clipped {
                if !icon_set {
                    if let Some(icon) = region.icon {
                        cx.set_cursor(icon);
                        icon_set = true;
                    }
                }

                self.hovered_regions.insert(region.token);

                if !cx.mouse_down() && !cx.right_mouse_down() {
                    if let Some(on_hover) = &region.on_hover {
                        cx.transform = region.transform;
                        on_hover(&mut cx);
                        self.hovered_regions.insert(region.token);
                    }
                }

                if cx.mouse_just_down() && !left_consumed {
                    if let Some(on_down) = &region.on_down {
                        cx.transform = region.transform;
                        on_down(&mut cx);

                        left_consumed = true;
                    }

                    if self.current_dragger.is_none() && region.on_drag.is_some() {
                        self.current_dragger = Some(region.token);
                    }

                    if self.current_clicker.is_none() && region.on_click.is_some() {
                        self.current_clicker = Some(region.token);
                    }
                }

                if cx.right_mouse_just_down() && !right_consumed {
                    if let Some(on_right_down) = &region.on_right_down {
                        cx.transform = region.transform;
                        on_right_down(&mut cx);

                        right_consumed = true;
                    }

                    if self.current_right_dragger.is_none() && region.on_right_drag.is_some() {
                        self.current_right_dragger = Some(region.token);
                    }

                    if self.current_right_clicker.is_none() && region.on_right_click.is_some() {
                        self.current_right_clicker = Some(region.token);
                    }
                }

                if cx.mouse_released() && !left_consumed {
                    if let Some(on_up) = &region.on_up {
                        cx.transform = region.transform;
                        on_up(&mut cx);
                    }

                    if let Some(on_click) = &region.on_click {
                        if !self.drag_min_reached {
                            if current_clicker == Some(region.token) {
                                cx.transform = region.transform;
                                on_click(&mut cx);
                                left_consumed = true;
                            }
                        }
                    }
                }

                if cx.right_mouse_released() && !right_consumed {
                    if let Some(on_right_up) = &region.on_right_up {
                        cx.transform = region.transform;
                        on_right_up(&mut cx);
                    }

                    if let Some(on_right_click) = &region.on_right_click {
                        if !self.right_drag_min_reached {
                            if current_right_clicker == Some(region.token) {
                                cx.transform = region.transform;
                                on_right_click(&mut cx);
                                right_consumed = true;
                            }
                        }
                    }
                }
            }

            if left_consumed && right_consumed {
                break;
            }
        }

        for region in self.mouse_regions.iter().rev() {
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

    pub fn draw_mouse_regions(&self, scene: &mut Scene) {
        let base_color = Color::new([1., 0., 0., 0.25]);
        let region_count = self.mouse_regions.len();
        for (i, region) in self.mouse_regions.iter().enumerate() {
            scene.fill(
                Fill::NonZero,
                Affine::IDENTITY,
                &Brush::Solid(
                    base_color.map_hue(|x| x + (i as f32 * 360.) / (region_count as f32)),
                ),
                None,
                &region.region,
            );
        }
    }
}
