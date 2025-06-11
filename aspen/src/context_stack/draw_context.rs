use std::{
    collections::{HashMap, HashSet},
    ops::Deref,
};

use parley::{layout::PositionedLayoutItem, Layout};
use vello::{
    kurbo::{Affine, BezPath, Line, Point, Rect, RoundedRect, Shape, Size, Stroke, Vec2},
    peniko::{BlendMode, Brush, Color, Fill},
    Scene,
};

use super::AttachedContext;

use crate::{
    element::{Element, ElementPointer},
    mouse_region::{MouseRegion, MouseRegionManager, RegionToken},
    token::Token,
};

pub struct DrawContext<'a> {
    context: AttachedContext<'a>,
    mouse_region_manager: &'a mut MouseRegionManager,
    mouse_region_count: usize,
    child_lookup: &'a HashMap<Token, HashSet<Token>>,
    // Lookup table for all element's transforms and sizes in element transform coordinates
    regions: &'a HashMap<Token, (Affine, Size)>,
    stroke_style: Stroke,
    stroke_brush: Brush,
    fill_brush: Brush,
    // Transform for this element computed during layout
    element_transform: Affine,
    // Transform list local to this element. Used to enable layer local transforms.
    local_transform_stack: Vec<Affine>,
    // Currently active clipping paths in window space
    clip_stack: Vec<BezPath>,
    scene: &'a mut Scene,
}

impl<'a> Deref for DrawContext<'a> {
    type Target = AttachedContext<'a>;

    fn deref(&self) -> &Self::Target {
        &self.context
    }
}

impl<'a> DrawContext<'a> {
    pub fn new(
        context: AttachedContext<'a>,
        mouse_region_manager: &'a mut MouseRegionManager,
        child_lookup: &'a HashMap<Token, HashSet<Token>>,
        regions: &'a HashMap<Token, (Affine, Size)>,
        scene: &'a mut Scene,
    ) -> DrawContext<'a> {
        DrawContext {
            context,
            mouse_region_manager,
            mouse_region_count: 0,
            child_lookup,
            regions,
            stroke_style: Stroke::new(2.),
            stroke_brush: Brush::Solid(Color::BLACK),
            fill_brush: Brush::Solid(Color::WHITE),
            element_transform: Affine::IDENTITY,
            local_transform_stack: vec![Affine::IDENTITY],
            clip_stack: vec![],
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
            self.current_transform(),
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
            self.current_transform(),
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
                self.current_transform(),
                rounded_rect.rect(),
                color,
                rounded_rect.radii().as_single_radius().unwrap(),
                std_dev,
            );
        } else {
            panic!("Blurred rect drawn without solid color brush");
        }
    }

    pub fn draw_layout_at(&mut self, layout: &Layout<Brush>, position: Point) {
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
        {
            // Clone most recent local transform onto the stack
            let local_transform = self.local_transform_stack.last().copied().unwrap();
            self.local_transform_stack.push(local_transform);
        }

        let transform = self.current_transform();
        self.scene
            .push_layer(BlendMode::default(), alpha, transform, clip);
        self.clip_stack.push(transform * clip.to_path(0.1));
    }

    pub fn pop_layer(&mut self) {
        self.local_transform_stack.pop();
        self.scene.pop_layer();
        self.clip_stack.pop();
        if self.local_transform_stack.is_empty() {
            panic!("Popped too many layers");
        }
    }

    pub fn element_transform(&self) -> Affine {
        self.element_transform
    }

    pub fn current_transform(&self) -> Affine {
        self.element_transform * *self.local_transform_stack.last().unwrap()
    }

    pub fn update_local_transform(&mut self, update: impl FnOnce(Affine) -> Affine) {
        let transform = self.local_transform_stack.last_mut().unwrap();
        *transform = update(*transform);
    }

    pub fn rotate(&mut self, radians: f64) {
        self.update_local_transform(|t| t.then_rotate(radians));
    }

    pub fn rotate_about(&mut self, radians: f64, center: Point) {
        self.update_local_transform(|t| t.then_rotate_about(radians, center));
    }

    pub fn scale(&mut self, scale: f64) {
        self.update_local_transform(|t| t.then_scale(scale));
    }

    pub fn scale_non_uniform(&mut self, scale_x: f64, scale_y: f64) {
        self.update_local_transform(|t| t.then_scale_non_uniform(scale_x, scale_y));
    }

    pub fn scale_about(&mut self, scale: f64, center: Point) {
        self.update_local_transform(|t| t.then_scale_about(scale, center));
    }

    pub fn translate(&mut self, offset: Vec2) {
        self.update_local_transform(|t| t.then_translate(offset));
    }

    pub fn transform(&mut self, transform: Affine) {
        self.update_local_transform(|t| t * transform);
    }

    pub fn mouse_position(&self) -> Option<Point> {
        self.actual_mouse_position()
            .map(|pos| self.current_transform().inverse() * pos)
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
            .map(|pos| self.current_transform().inverse() * pos)
    }

    pub fn previous_mouse_position_relative_to<Other: Element>(
        &self,
        other: &ElementPointer<Other>,
    ) -> Option<Point> {
        self.regions
            .get(&other.token())
            .map(|(transform, _)| {
                self.actual_previous_mouse_position()
                    .map(|prev| transform.inverse() * prev)
            })
            .expect(&format!(
                "Layout must not have been completed for this element before drawing: {:?}",
                other.token()
            ))
    }

    pub fn mouse_delta(&self) -> Option<Vec2> {
        self.mouse_position()
            .zip(self.previous_mouse_position())
            .map(|(pos, prev)| pos - prev)
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
        self.current_transform()
            .inverse()
            .transform_rect_bbox(self.actual_window_rect())
    }

    pub fn window_shape(&self) -> BezPath {
        self.current_transform().inverse() * self.actual_window_rect().to_path(0.1)
    }

    pub fn mouse_region(&mut self, region: impl Shape) -> &mut MouseRegion {
        self.mouse_region_manager.add_region(MouseRegion::new(
            RegionToken {
                token: self.context.token(),
                index: {
                    let index = self.mouse_region_count;
                    self.mouse_region_count += 1;
                    index
                },
            },
            self.children().clone(),
            region,
            self.current_transform(),
            self.clip_stack.clone(),
        ))
    }

    pub fn region(&self) -> Rect {
        self.regions
            .get(&self.context.token())
            .map(|(_, size)| Rect::from_origin_size(Point::ZERO, *size))
            .expect("Layout must not have been completed before drawing")
    }

    pub fn region_of<Other: Element>(&self, other: &ElementPointer<Other>) -> BezPath {
        self.regions
            .get(&other.token())
            .map(|(transform, size)| {
                *transform * Rect::from_origin_size(Point::ZERO, *size).to_path(0.1)
            })
            .expect(&format!(
                "Layout must not have been completed for this element before drawing: {:?}",
                other.token()
            ))
    }

    pub fn transform_of<Other: Element>(&self, other: &ElementPointer<Other>) -> Affine {
        self.transform_by_token(other.token())
    }

    fn transform_by_token(&self, other_token: Token) -> Affine {
        self.regions
            .get(&other_token)
            .map(|(transform, _)| *transform)
            .expect(&format!(
                "Layout must not have been completed for this element before drawing: {:?}",
                other_token
            ))
    }

    fn any_in_progress_mouse_regions_recursive(&self, token: Token) -> bool {
        if self.mouse_region_manager.token_currently_tracked(token) {
            return true;
        }

        if let Some(children) = self.child_lookup.get(&token) {
            for token in children {
                if self.any_in_progress_mouse_regions_recursive(*token) {
                    return true;
                }
            }
            false
        } else {
            false
        }
    }

    pub fn any_in_progress_mouse_regions(&self) -> bool {
        self.any_in_progress_mouse_regions_recursive(self.token())
    }

    pub(crate) fn child<'b>(
        &'b mut self,
        element_token: Token,
        element_children: Vec<Token>,
    ) -> DrawContext<'b>
    where
        'a: 'b,
    {
        let element_transform = self.element_transform * self.transform_by_token(element_token);
        let child_cx: AttachedContext<'b> = self.context.child(element_token, element_children);
        DrawContext::<'b> {
            context: child_cx,
            mouse_region_manager: self.mouse_region_manager,
            mouse_region_count: 0,
            child_lookup: self.child_lookup,
            regions: self.regions,
            stroke_style: self.stroke_style.clone(),
            stroke_brush: self.stroke_brush.clone(),
            fill_brush: self.fill_brush.clone(),
            element_transform,
            local_transform_stack: vec![Affine::IDENTITY],
            clip_stack: self.clip_stack.clone(),
            scene: self.scene,
        }
    }
}
