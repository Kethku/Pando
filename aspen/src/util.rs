use vello::{
    kurbo::{Affine, Point, Rect, Vec2},
    peniko::{color::HueDirection, Color},
};

pub trait Mixable: Sized {
    fn add(&self, other: &Self) -> Self;
    fn sub(&self, other: &Self) -> Self;
    fn scale(&self, factor: f64) -> Self;
    fn mix(&self, other: &Self, factor: f64) -> Self {
        let factor = factor.clamp(0., 1.);
        self.add(&other.sub(&self).scale(factor))
    }
}

impl Mixable for f64 {
    fn add(&self, other: &Self) -> Self {
        self + other
    }

    fn sub(&self, other: &Self) -> Self {
        self - other
    }

    fn scale(&self, factor: f64) -> Self {
        self * factor
    }
}

impl Mixable for Color {
    fn add(&self, other: &Self) -> Self {
        *self + *other
    }

    fn sub(&self, other: &Self) -> Self {
        *self - *other
    }

    fn scale(&self, factor: f64) -> Self {
        *self * factor as f32
    }

    fn mix(&self, other: &Self, factor: f64) -> Self {
        self.lerp(*other, (factor as f32).clamp(0., 1.), HueDirection::Shorter)
    }
}

impl Mixable for Rect {
    fn add(&self, other: &Self) -> Self {
        Rect::from_origin_size(
            self.origin() + other.origin().to_vec2(),
            self.size() + other.size(),
        )
    }
    fn sub(&self, other: &Self) -> Self {
        Rect::from_origin_size(
            self.origin() - other.origin().to_vec2(),
            self.size() - other.size(),
        )
    }
    fn scale(&self, factor: f64) -> Self {
        Rect::from_origin_size(
            (self.origin().to_vec2() * factor).to_point(),
            self.size() * factor,
        )
    }
}

pub trait RectExt {
    fn corners(&self) -> [Point; 4];
}

impl RectExt for Rect {
    fn corners(&self) -> [Point; 4] {
        [
            Point::new(self.x0, self.y0),
            Point::new(self.x1, self.y0),
            Point::new(self.x1, self.y1),
            Point::new(self.x0, self.y1),
        ]
    }
}

pub trait PointExt {
    fn snap(&self) -> Point;
}

impl PointExt for Point {
    fn snap(&self) -> Point {
        self.floor() + Vec2::new(0.5, 0.5)
    }
}

pub trait AffineExt {
    fn unskewed_scale(&self) -> Vec2;
}

impl AffineExt for Affine {
    fn unskewed_scale(&self) -> Vec2 {
        let coeffs = self.as_coeffs();
        Vec2::new(
            (coeffs[0] * coeffs[0] + coeffs[2] * coeffs[2]).sqrt(),
            (coeffs[1] * coeffs[1] + coeffs[3] * coeffs[3]).sqrt(),
        )
    }
}
