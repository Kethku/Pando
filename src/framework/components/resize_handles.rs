use glamour::prelude::*;

use vide::winit::window::{CursorIcon, ResizeDirection};

use crate::framework::{
    context::{DrawContext, LayoutContext},
    element::{Element, ElementPointer},
    geometry::*,
    mouse_region::MouseRegion,
    token::Token,
};

const EDGE_DEPTH: f32 = 8.;
const CORNER_DEPTH: f32 = 12.;

pub struct ResizeHandles {
    edge_handles: [ElementPointer<EdgeHandle>; 4],
    corner_handles: [ElementPointer<CornerHandle>; 4],
}

impl ResizeHandles {
    pub fn new() -> ElementPointer<Self> {
        ElementPointer::new(Self {
            edge_handles: Cardinal::ALL.map(|direction| EdgeHandle::new(direction)),
            corner_handles: Ordinal::ALL.map(|direction| CornerHandle::new(direction)),
        })
    }
}

impl Element for ResizeHandles {
    fn layout(&mut self, min: Size2, max: Size2, cx: &mut LayoutContext) -> Size2 {
        for handle in self.edge_handles.iter_mut() {
            handle.layout(min, max, cx).position(Point2::ZERO, cx);
        }

        for handle in self.corner_handles.iter_mut() {
            handle.layout(min, max, cx).position(Point2::ZERO, cx);
        }

        max
    }

    fn draw(&self, cx: &mut DrawContext) {
        for handle in &self.edge_handles {
            handle.draw(cx);
        }

        for handle in &self.corner_handles {
            handle.draw(cx);
        }
    }
}

struct EdgeHandle {
    token: Token,
    direction: Cardinal,
}

impl EdgeHandle {
    fn new(direction: Cardinal) -> ElementPointer<Self> {
        ElementPointer::new(Self {
            token: Token::new(),
            direction,
        })
    }

    fn resize_direction(&self) -> ResizeDirection {
        match self.direction {
            Cardinal::North => ResizeDirection::North,
            Cardinal::East => ResizeDirection::East,
            Cardinal::South => ResizeDirection::South,
            Cardinal::West => ResizeDirection::West,
        }
    }

    fn icon(&self) -> CursorIcon {
        match self.direction {
            Cardinal::North => CursorIcon::NResize,
            Cardinal::East => CursorIcon::EResize,
            Cardinal::South => CursorIcon::SResize,
            Cardinal::West => CursorIcon::WResize,
        }
    }
}

impl Element for EdgeHandle {
    fn layout(&mut self, _min: Size2, max: Size2, _cx: &mut LayoutContext) -> Size2 {
        max
    }

    fn draw(&self, cx: &mut DrawContext) {
        let rect = cx.window_rect();

        cx.add_mouse_region(
            MouseRegion::new(self.token, rect.edge_rect(self.direction, EDGE_DEPTH))
                .with_icon(self.icon())
                .on_down({
                    let direction = self.resize_direction();
                    move |cx| cx.drag_resize_window(direction)
                }),
        );
    }
}

struct CornerHandle {
    token: Token,
    direction: Ordinal,
}

impl CornerHandle {
    fn new(direction: Ordinal) -> ElementPointer<Self> {
        ElementPointer::new(Self {
            token: Token::new(),
            direction,
        })
    }

    fn resize_direction(&self) -> ResizeDirection {
        match self.direction {
            Ordinal::NorthWest => ResizeDirection::NorthWest,
            Ordinal::NorthEast => ResizeDirection::NorthEast,
            Ordinal::SouthEast => ResizeDirection::SouthEast,
            Ordinal::SouthWest => ResizeDirection::SouthWest,
        }
    }

    fn icon(&self) -> CursorIcon {
        match self.direction {
            Ordinal::NorthWest => CursorIcon::NwResize,
            Ordinal::NorthEast => CursorIcon::NeResize,
            Ordinal::SouthEast => CursorIcon::SeResize,
            Ordinal::SouthWest => CursorIcon::SwResize,
        }
    }
}

impl Element for CornerHandle {
    fn layout(&mut self, _min: Size2, max: Size2, _cx: &mut LayoutContext) -> Size2 {
        max
    }

    fn draw(&self, cx: &mut DrawContext) {
        let rect = cx.window_rect();

        cx.add_mouse_region(
            MouseRegion::new(self.token, rect.corner_square(self.direction, CORNER_DEPTH))
                .with_icon(self.icon())
                .on_down({
                    let direction = self.resize_direction();
                    move |cx| cx.drag_resize_window(direction)
                }),
        );
    }
}
