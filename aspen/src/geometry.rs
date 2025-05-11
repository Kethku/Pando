use vello::kurbo::{Point, Rect, Size};

#[derive(Copy, Clone)]
pub enum Cardinal {
    North,
    South,
    East,
    West,
}

impl Cardinal {
    pub const ALL: [Cardinal; 4] = [
        Cardinal::North,
        Cardinal::South,
        Cardinal::East,
        Cardinal::West,
    ];
}

#[derive(Copy, Clone)]
pub enum Ordinal {
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl Ordinal {
    pub const ALL: [Ordinal; 4] = [
        Ordinal::NorthEast,
        Ordinal::NorthWest,
        Ordinal::SouthEast,
        Ordinal::SouthWest,
    ];
}

pub trait RectExt {
    fn edge_rect(&self, cardinal: Cardinal, depth: f64) -> Rect;
    fn corner_square(&self, ordinal: Ordinal, depth: f64) -> Rect;
}

impl RectExt for Rect {
    fn edge_rect(&self, cardinal: Cardinal, depth: f64) -> Rect {
        match cardinal {
            Cardinal::North => Rect::from_origin_size(
                Point::new(self.min_x(), self.min_y()),
                Size::new(self.width(), depth),
            ),
            Cardinal::South => Rect::from_origin_size(
                Point::new(self.min_x(), self.max_y() - depth),
                Size::new(self.width(), depth),
            ),
            Cardinal::East => Rect::from_origin_size(
                Point::new(self.max_x() - depth, self.min_y()),
                Size::new(depth, self.height()),
            ),
            Cardinal::West => Rect::from_origin_size(
                Point::new(self.min_x(), self.min_y()),
                Size::new(depth, self.height()),
            ),
        }
    }

    fn corner_square(&self, ordinal: Ordinal, depth: f64) -> Rect {
        match ordinal {
            Ordinal::NorthEast => Rect::from_origin_size(
                Point::new(self.max_x() - depth, self.min_y()),
                Size::new(depth, depth),
            ),
            Ordinal::NorthWest => Rect::from_origin_size(
                Point::new(self.min_x(), self.min_y()),
                Size::new(depth, depth),
            ),
            Ordinal::SouthEast => Rect::from_origin_size(
                Point::new(self.max_x() - depth, self.max_y() - depth),
                Size::new(depth, depth),
            ),
            Ordinal::SouthWest => Rect::from_origin_size(
                Point::new(self.min_x(), self.max_y() - depth),
                Size::new(depth, depth),
            ),
        }
    }
}
