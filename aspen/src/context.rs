use std::{
    collections::{HashMap, HashSet},
    ops::Deref,
    sync::Arc,
};

use mockall::*;
use parley::{layout::PositionedLayoutItem, Layout};
use vello::{
    kurbo::{Affine, Line, Point, Rect, RoundedRect, Shape, Size, Stroke, Vec2},
    peniko::{BlendMode, Brush, Color, Fill},
    Scene,
};
use winit::{
    event_loop::ActiveEventLoop,
    window::{Cursor, CursorIcon, ResizeDirection, Window},
};

use crate::{
    element::{Element, ElementPointer},
    mouse_region::{MouseRegion, MouseRegionManager},
    token::Token,
};

pub struct EventState {
    pub mouse_position: Point,
    pub previous_mouse_position: Point,
    pub window_size: Size,
    pub mouse_down: bool,
    pub was_mouse_down: bool,
    pub scroll_delta: Vec2,
}

impl EventState {
    pub fn new() -> Self {
        Self {
            mouse_position: Point::new(0., 0.),
            previous_mouse_position: Point::new(0., 0.),
            window_size: Size::new(0., 0.),
            mouse_down: false,
            was_mouse_down: false,
            scroll_delta: Vec2::new(0., 0.),
        }
    }

    pub fn next_frame(&mut self) {
        self.was_mouse_down = self.mouse_down;
        self.previous_mouse_position = self.mouse_position;
        self.scroll_delta = Vec2::new(0., 0.);
    }

    pub fn mouse_down(&self) -> bool {
        self.mouse_down
    }

    pub fn was_mouse_down(&self) -> bool {
        self.was_mouse_down
    }

    pub fn mouse_released(&self) -> bool {
        !self.mouse_down && self.was_mouse_down
    }

    pub fn mouse_just_down(&self) -> bool {
        self.mouse_down && !self.was_mouse_down
    }

    pub fn mouse_held(&self) -> bool {
        self.mouse_down && self.was_mouse_down
    }

    pub fn actual_mouse_position(&self) -> Point {
        self.mouse_position
    }

    pub fn actual_previous_mouse_position(&self) -> Point {
        self.previous_mouse_position
    }

    pub fn actual_mouse_delta(&self) -> Vec2 {
        self.mouse_position - self.previous_mouse_position
    }

    pub fn scroll_delta(&self) -> Vec2 {
        self.scroll_delta
    }

    pub fn actual_window_size(&self) -> Size {
        self.window_size
    }

    pub fn actual_window_rect(&self) -> Rect {
        Rect::from_origin_size(Point::new(0., 0.), self.window_size)
    }
}

pub struct Context<'a> {
    event_state: &'a EventState,
    event_loop: &'a dyn ContextEventLoop,
    window: Arc<dyn ContextWindow>,
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
        event_loop: &'a dyn ContextEventLoop,
        window: Arc<dyn ContextWindow>,
        element_token: Token,
    ) -> Context<'a> {
        Context {
            event_state,
            event_loop,
            window,
            element_token,
        }
    }

    pub fn close(&self) {
        self.event_loop.exit();
    }

    pub fn toggle_maximized(&self) {
        self.window.set_maximized(!self.window.is_maximized());
    }

    pub fn is_maximized(&self) -> bool {
        self.window.is_maximized()
    }

    pub fn minimize(&self) {
        self.window.set_minimized(true);
    }

    pub fn is_minimized(&self) -> bool {
        self.window.is_minimized().unwrap_or_default()
    }

    pub fn drag_window(&self) {
        self.window.drag_window().expect("Could not drag window");
    }

    pub fn drag_resize_window(&self, direction: ResizeDirection) {
        self.window
            .drag_resize_window(direction)
            .expect("Could not drag resize window");
    }

    pub fn set_cursor(&self, icon: CursorIcon) {
        self.window.set_cursor(Cursor::Icon(icon));
    }

    pub fn token(&self) -> Token {
        self.element_token
    }

    pub fn child<'b>(&self, element_token: Token) -> Context<'b>
    where
        'a: 'b,
    {
        Context {
            event_state: self.event_state,
            event_loop: self.event_loop,
            window: self.window.clone(),
            element_token,
        }
    }
}

#[automock]
pub trait ContextEventLoop {
    fn exit(&self);
}

impl ContextEventLoop for ActiveEventLoop {
    fn exit(&self) {
        self.exit();
    }
}

#[automock]
pub trait ContextWindow {
    fn set_maximized(&self, maximized: bool);
    fn is_maximized(&self) -> bool;
    fn set_minimized(&self, minimized: bool);
    fn is_minimized(&self) -> Option<bool>;
    fn drag_window(&self) -> Result<(), String>;
    fn drag_resize_window(&self, direction: ResizeDirection) -> Result<(), String>;
    fn set_cursor(&self, cursor: Cursor);
    fn request_redraw(&self);
}

impl ContextWindow for Window {
    fn set_maximized(&self, maximized: bool) {
        self.set_maximized(maximized);
    }

    fn is_maximized(&self) -> bool {
        self.is_maximized()
    }

    fn set_minimized(&self, minimized: bool) {
        self.set_minimized(minimized);
    }

    fn is_minimized(&self) -> Option<bool> {
        self.is_minimized()
    }

    fn drag_window(&self) -> Result<(), String> {
        self.drag_window().map_err(|e| e.to_string())
    }

    fn drag_resize_window(&self, direction: ResizeDirection) -> Result<(), String> {
        self.drag_resize_window(direction)
            .map_err(|e| e.to_string())
    }

    fn set_cursor(&self, cursor: Cursor) {
        self.set_cursor(cursor);
    }

    fn request_redraw(&self) {
        self.request_redraw();
    }
}

pub struct EventContext<'a> {
    context: &'a Context<'a>,
    redraw_requested: &'a mut bool,
    // Used when a drag just crossed the min threshold to report as a drag so that the dragger can
    // get a delta value that includes the threshold distance for the first mouse delta.
    //
    // Note: this presents some slight weirdness because mouse_delta will be larger than the actual
    // computed delta but so be it.
    pub(crate) delta_correction: Option<Vec2>,
    pub(crate) transform: Affine,
}

impl<'a> Deref for EventContext<'a> {
    type Target = Context<'a>;

    fn deref(&self) -> &Self::Target {
        &self.context
    }
}

impl<'a> EventContext<'a> {
    pub fn new(context: &'a Context<'a>, redraw_requested: &'a mut bool) -> EventContext<'a> {
        EventContext {
            context,
            redraw_requested,
            delta_correction: None,
            transform: Affine::IDENTITY,
        }
    }

    pub fn request_redraw(&mut self) {
        *self.redraw_requested = true;
    }

    pub fn mouse_position(&self) -> Point {
        self.transform.inverse() * self.actual_mouse_position()
    }

    pub fn previous_mouse_position(&self) -> Point {
        self.transform.inverse() * self.actual_previous_mouse_position()
    }

    pub fn mouse_delta(&self) -> Vec2 {
        if let Some(delta) = self.delta_correction {
            delta
        } else {
            self.mouse_position() - self.previous_mouse_position()
        }
    }

    pub fn window_bounding_box(&self) -> Rect {
        self.transform
            .inverse()
            .transform_rect_bbox(self.actual_window_rect())
    }
}

pub struct UpdateContext<'a> {
    context: Context<'a>,
    mouse_region_manager: &'a mut MouseRegionManager,
    redraw_requested: &'a mut bool,
}

impl<'a> Deref for UpdateContext<'a> {
    type Target = Context<'a>;

    fn deref(&self) -> &Self::Target {
        &self.context
    }
}

impl<'a> UpdateContext<'a> {
    pub fn new(
        context: Context<'a>,
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

    pub fn child<'b>(&'b mut self, element_token: Token) -> UpdateContext<'b>
    where
        'a: 'b,
    {
        let child_cx: Context<'b> = self.context.child(element_token);
        UpdateContext {
            context: child_cx,
            mouse_region_manager: self.mouse_region_manager,
            redraw_requested: self.redraw_requested,
        }
    }
}

pub struct LayoutContext<'a> {
    context: Context<'a>,
    regions: &'a mut HashMap<Token, Rect>,
    children: &'a mut HashMap<Token, HashSet<Token>>,
}

impl<'a> Deref for LayoutContext<'a> {
    type Target = Context<'a>;

    fn deref(&self) -> &Self::Target {
        &self.context
    }
}

impl<'a> LayoutContext<'a> {
    pub fn new(
        context: Context<'a>,
        regions: &'a mut HashMap<Token, Rect>,
        children: &'a mut HashMap<Token, HashSet<Token>>,
    ) -> LayoutContext<'a> {
        LayoutContext {
            context,
            regions,
            children,
        }
    }

    pub fn add_region(&mut self, token: Token, rect: Rect) {
        self.regions.insert(token, rect);
    }

    pub fn child<'b>(&'b mut self, token: Token) -> LayoutContext<'b>
    where
        'a: 'b,
    {
        self.children.entry(self.token()).or_default().insert(token);
        let child_cx: Context<'b> = self.context.child(token);
        LayoutContext::<'b> {
            context: child_cx,
            regions: self.regions,
            children: self.children,
        }
    }

    pub fn translate_descendants(&mut self, token: Token, offset: Vec2) {
        if let Some(children) = self.children.get(&token).cloned() {
            for child in children.iter() {
                if let Some(region) = self.regions.get_mut(child) {
                    *region = *region + offset;
                    self.translate_descendants(*child, offset);
                }
            }
        }
    }
}

pub struct DrawContext<'a> {
    context: Context<'a>,
    mouse_region_manager: &'a mut MouseRegionManager,
    regions: &'a HashMap<Token, Rect>,
    stroke_style: Stroke,
    stroke_brush: Brush,
    fill_brush: Brush,
    transform_stack: Vec<Affine>,
    scene: &'a mut Scene,
}

impl<'a> Deref for DrawContext<'a> {
    type Target = Context<'a>;

    fn deref(&self) -> &Self::Target {
        &self.context
    }
}

impl<'a> DrawContext<'a> {
    pub fn new(
        context: Context<'a>,
        mouse_region_manager: &'a mut MouseRegionManager,
        regions: &'a HashMap<Token, Rect>,
        scene: &'a mut Scene,
    ) -> DrawContext<'a> {
        DrawContext {
            context,
            mouse_region_manager,
            regions,
            stroke_style: Stroke::new(2.),
            stroke_brush: Brush::Solid(Color::BLACK),
            fill_brush: Brush::Solid(Color::WHITE),
            transform_stack: vec![Affine::IDENTITY],
            scene,
        }
    }

    pub fn set_stroke_style(&mut self, stroke_style: Stroke) {
        self.stroke_style = stroke_style;
    }

    pub fn set_stroke_brush(&mut self, stroke_brush: Brush) {
        self.stroke_brush = stroke_brush;
    }

    pub fn stroke(&mut self, shape: &impl Shape) {
        self.scene.stroke(
            &self.stroke_style,
            self.transform_stack.last().copied().unwrap(),
            &self.stroke_brush,
            None,
            shape,
        );
    }

    pub fn set_fill_brush(&mut self, fill_brush: Brush) {
        self.fill_brush = fill_brush;
    }

    pub fn fill(&mut self, shape: &impl Shape) {
        self.scene.fill(
            Fill::NonZero,
            self.transform_stack.last().copied().unwrap(),
            &self.fill_brush,
            None,
            shape,
        );
    }

    pub fn stroked_fill(&mut self, shape: &impl Shape) {
        self.fill(shape);
        self.stroke(shape);
    }

    pub fn blurred(&mut self, rounded_rect: RoundedRect, std_dev: f64) {
        if let Brush::Solid(color) = self.fill_brush {
            self.scene.draw_blurred_rounded_rect(
                self.transform_stack.last().copied().unwrap(),
                rounded_rect.rect(),
                color,
                rounded_rect.radii().as_single_radius().unwrap(),
                std_dev,
            );
        } else {
            panic!("Blurred rect drawn without solid color brush");
        }
    }

    pub fn draw_layout(&mut self, layout: &Layout<Brush>, position: Point) {
        let transform = self.current_transform().pre_translate(position.to_vec2());
        for line in layout.lines() {
            for item in line.items() {
                let PositionedLayoutItem::GlyphRun(glyph_run) = item else {
                    continue;
                };
                let style = glyph_run.style();
                if let Some(underline) = &style.underline {
                    let underline_brush = &style.brush;
                    let run_metrics = glyph_run.run().metrics();

                    let offset = match underline.offset {
                        Some(offset) => offset,
                        None => run_metrics.underline_offset,
                    };
                    let width = match underline.size {
                        Some(size) => size,
                        None => run_metrics.underline_size,
                    };

                    // The `offset` is the distance from the baseline to the top of the underline
                    // so we move the line down by half the width
                    // Remember that we are using a y-down coordinate system
                    // If there's a custom width, because this is an underline, we want the custom
                    // width to go down from the default expectation
                    let y = glyph_run.baseline() - offset + width / 2.;

                    let line = Line::new(
                        (glyph_run.offset() as f64, y as f64),
                        ((glyph_run.offset() + glyph_run.advance()) as f64, y as f64),
                    );
                    self.scene.stroke(
                        &Stroke::new(width.into()),
                        transform,
                        underline_brush,
                        None,
                        &line,
                    );
                }

                let mut x = glyph_run.offset();
                let y = glyph_run.baseline();
                let run = glyph_run.run();
                let font = run.font();
                let font_size = run.font_size();
                let synthesis = run.synthesis();
                let glyph_xform = synthesis
                    .skew()
                    .map(|angle| Affine::skew(angle.to_radians().tan() as f64, 0.0));
                self.scene
                    .draw_glyphs(font)
                    .transform(transform)
                    .brush(&style.brush)
                    .hint(true)
                    .glyph_transform(glyph_xform)
                    .font_size(font_size)
                    .normalized_coords(run.normalized_coords())
                    .draw(
                        Fill::NonZero,
                        glyph_run.glyphs().map(|glyph| {
                            let gx = x + glyph.x;
                            let gy = y - glyph.y;
                            x += glyph.advance;
                            vello::Glyph {
                                id: glyph.id as _,
                                x: gx,
                                y: gy,
                            }
                        }),
                    );
                if let Some(strikethrough) = &style.strikethrough {
                    let strikethrough_brush = &style.brush;
                    let run_metrics = glyph_run.run().metrics();
                    let offset = match strikethrough.offset {
                        Some(offset) => offset,
                        None => run_metrics.strikethrough_offset,
                    };
                    let width = match strikethrough.size {
                        Some(size) => size,
                        None => run_metrics.strikethrough_size,
                    };
                    // The `offset` is the distance from the baseline to the *top* of the strikethrough
                    // so we calculate the middle y-position of the strikethrough based on the font's
                    // standard strikethrough width.
                    // Remember that we are using a y-down coordinate system
                    let y = glyph_run.baseline() - offset + run_metrics.strikethrough_size / 2.;

                    let line = Line::new(
                        (glyph_run.offset() as f64, y as f64),
                        ((glyph_run.offset() + glyph_run.advance()) as f64, y as f64),
                    );
                    self.scene.stroke(
                        &Stroke::new(width.into()),
                        transform,
                        strikethrough_brush,
                        None,
                        &line,
                    );
                }
            }
        }
    }

    pub fn push_layer(&mut self, alpha: f32, clip: &impl Shape) {
        let transform = self.current_transform();
        self.transform_stack.push(transform);
        self.scene
            .push_layer(BlendMode::default(), alpha, transform, clip);
    }

    pub fn pop_layer(&mut self) {
        self.transform_stack.pop();
        self.scene.pop_layer();
        if self.transform_stack.is_empty() {
            panic!("Popped too many layers");
        }
    }

    pub fn current_transform(&self) -> Affine {
        self.transform_stack.last().copied().unwrap()
    }

    pub fn update_transform(&mut self, update: impl FnOnce(Affine) -> Affine) {
        let transform = self.transform_stack.last_mut().unwrap();
        *transform = update(*transform);
    }

    pub fn rotate(&mut self, radians: f64) {
        self.update_transform(|t| t.then_rotate(radians));
    }

    pub fn rotate_about(&mut self, radians: f64, center: Point) {
        self.update_transform(|t| t.then_rotate_about(radians, center));
    }

    pub fn scale(&mut self, scale: f64) {
        self.update_transform(|t| t.then_scale(scale));
    }

    pub fn scale_non_uniform(&mut self, scale_x: f64, scale_y: f64) {
        self.update_transform(|t| t.then_scale_non_uniform(scale_x, scale_y));
    }

    pub fn scale_about(&mut self, scale: f64, center: Point) {
        self.update_transform(|t| t.then_scale_about(scale, center));
    }

    pub fn translate(&mut self, offset: Vec2) {
        self.update_transform(|t| t.then_translate(offset));
    }

    pub fn transform(&mut self, transform: Affine) {
        self.update_transform(|t| t * transform);
    }

    pub fn mouse_position(&self) -> Point {
        self.current_transform().inverse() * self.actual_mouse_position()
    }

    pub fn previous_mouse_position(&self) -> Point {
        self.current_transform().inverse() * self.actual_previous_mouse_position()
    }

    pub fn mouse_delta(&self) -> Vec2 {
        self.mouse_position() - self.previous_mouse_position()
    }

    pub fn window_bounding_box(&self) -> Rect {
        self.current_transform()
            .inverse()
            .transform_rect_bbox(self.actual_window_rect())
    }

    pub fn mouse_region(&mut self, region: impl Shape) -> &mut MouseRegion {
        self.mouse_region_manager.add_region(MouseRegion::new(
            self.context.token(),
            region,
            self.current_transform(),
        ))
    }

    pub fn add_mouse_region(&mut self, mouse_region: MouseRegion) {
        self.mouse_region_manager.add_region(mouse_region);
    }

    pub fn request_redraw(&self) {
        self.context.window.request_redraw();
    }

    pub fn region(&self) -> Rect {
        self.regions
            .get(&self.context.token())
            .copied()
            .expect("Layout must not have been completed before drawing")
    }

    pub fn child_region<Child: Element>(&self, child: &ElementPointer<Child>) -> Rect {
        self.regions
            .get(&child.token())
            .copied()
            .expect("Layout must not have been completed for this child before drawing")
    }

    pub fn child<'b>(&'b mut self, token: Token) -> DrawContext<'b>
    where
        'a: 'b,
    {
        let child_cx: Context<'b> = self.context.child(token);
        DrawContext::<'b> {
            context: child_cx,
            mouse_region_manager: self.mouse_region_manager,
            regions: self.regions,
            stroke_style: self.stroke_style.clone(),
            stroke_brush: self.stroke_brush.clone(),
            fill_brush: self.fill_brush.clone(),
            transform_stack: self.transform_stack.clone(),
            scene: self.scene,
        }
    }
}
