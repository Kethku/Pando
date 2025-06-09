use vello::kurbo::{Affine, Size};
use winit::window::{CursorIcon, ResizeDirection};

use crate::{
    context_stack::{DrawContext, LayoutContext},
    element::{Element, ElementPointer},
    geometry::*,
};

const EDGE_DEPTH: f64 = 8.;
const CORNER_DEPTH: f64 = 12.;

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
    fn layout(&mut self, min: Size, max: Size, cx: &mut LayoutContext) -> Size {
        for handle in self.edge_handles.iter_mut() {
            handle.layout(min, max, cx).position(Affine::IDENTITY, cx);
        }

        for handle in self.corner_handles.iter_mut() {
            handle.layout(min, max, cx).position(Affine::IDENTITY, cx);
        }

        max
    }

    fn draw(&self, cx: &mut DrawContext) {
        if !cx.is_maximized() {
            for handle in &self.edge_handles {
                handle.draw(cx);
            }

            for handle in &self.corner_handles {
                handle.draw(cx);
            }
        }
    }
}

struct EdgeHandle {
    direction: Cardinal,
}

impl EdgeHandle {
    fn new(direction: Cardinal) -> ElementPointer<Self> {
        ElementPointer::new(Self { direction })
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
    fn layout(&mut self, _min: Size, max: Size, _cx: &mut LayoutContext) -> Size {
        max
    }

    fn draw(&self, cx: &mut DrawContext) {
        let rect = cx.actual_window_rect();

        cx.mouse_region(rect.edge_rect(self.direction, EDGE_DEPTH))
            .with_icon(self.icon())
            .on_down({
                let direction = self.resize_direction();
                move |cx| cx.drag_resize_window(direction)
            });
    }
}

struct CornerHandle {
    direction: Ordinal,
}

impl CornerHandle {
    fn new(direction: Ordinal) -> ElementPointer<Self> {
        ElementPointer::new(Self { direction })
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
    fn layout(&mut self, _min: Size, max: Size, _cx: &mut LayoutContext) -> Size {
        max
    }

    fn draw(&self, cx: &mut DrawContext) {
        let rect = cx.actual_window_rect();

        cx.mouse_region(rect.corner_square(self.direction, CORNER_DEPTH))
            .with_icon(self.icon())
            .on_down({
                let direction = self.resize_direction();
                move |cx| cx.drag_resize_window(direction)
            });
    }
}
