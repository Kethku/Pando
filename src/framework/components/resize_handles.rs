use vide::winit::window::{CursorIcon, ResizeDirection};

use crate::framework::{
    context::DrawContext, geometry::*, mouse_region::MouseRegion, token::Token,
};

const EDGE_DEPTH: f32 = 8.;
const CORNER_DEPTH: f32 = 12.;

pub struct ResizeHandles {
    edge_handles: [EdgeHandle; 4],
    corner_handles: [CornerHandle; 4],
}

impl ResizeHandles {
    pub fn new() -> Self {
        Self {
            edge_handles: Cardinal::ALL.map(|direction| EdgeHandle::new(direction)),
            corner_handles: Ordinal::ALL.map(|direction| CornerHandle::new(direction)),
        }
    }

    pub fn draw(&self, cx: &mut DrawContext) {
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
    fn new(direction: Cardinal) -> Self {
        Self {
            token: Token::new(),
            direction,
        }
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

struct CornerHandle {
    token: Token,
    direction: Ordinal,
}

impl CornerHandle {
    fn new(direction: Ordinal) -> Self {
        Self {
            token: Token::new(),
            direction,
        }
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
