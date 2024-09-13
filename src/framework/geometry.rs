use glamour::prelude::*;

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
    fn edge_rect(&self, cardinal: Cardinal, depth: f32) -> Rect;
    fn corner_square(&self, ordinal: Ordinal, depth: f32) -> Rect;
}

impl RectExt for Rect {
    fn edge_rect(&self, cardinal: Cardinal, depth: f32) -> Rect {
        match cardinal {
            Cardinal::North => Rect::new(
                point2!(self.min().x, self.min().y),
                size!(self.width(), depth),
            ),
            Cardinal::South => Rect::new(
                point2!(self.min().x, self.max().y - depth),
                size!(self.width(), depth),
            ),
            Cardinal::East => Rect::new(
                point2!(self.max().x - depth, self.min().y),
                size!(depth, self.height()),
            ),
            Cardinal::West => Rect::new(
                point2!(self.min().x, self.min().y),
                size!(depth, self.height()),
            ),
        }
    }

    fn corner_square(&self, ordinal: Ordinal, depth: f32) -> Rect {
        match ordinal {
            Ordinal::NorthEast => Rect::new(
                point2!(self.max().x - depth, self.min().y),
                size!(depth, depth),
            ),
            Ordinal::NorthWest => {
                Rect::new(point2!(self.min().x, self.min().y), size!(depth, depth))
            }
            Ordinal::SouthEast => Rect::new(
                point2!(self.max().x - depth, self.max().y - depth),
                size!(depth, depth),
            ),
            Ordinal::SouthWest => Rect::new(
                point2!(self.min().x, self.max().y - depth),
                size!(depth, depth),
            ),
        }
    }
}
