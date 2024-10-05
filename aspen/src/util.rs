use vide::{
    palette::{FromColor, IntoColor, Oklaba},
    prelude::*,
};

pub trait Mixable: Sized {
    fn add(&self, other: &Self) -> Self;
    fn sub(&self, other: &Self) -> Self;
    fn scale(&self, factor: f32) -> Self;
    fn mix(&self, other: &Self, factor: f32) -> Self {
        let factor = factor.clamp(0., 1.);
        self.add(&other.sub(&self).scale(factor))
    }
}

impl Mixable for f32 {
    fn add(&self, other: &Self) -> Self {
        self + other
    }

    fn sub(&self, other: &Self) -> Self {
        self - other
    }

    fn scale(&self, factor: f32) -> Self {
        self * factor
    }
}

impl Mixable for Srgba {
    fn add(&self, other: &Self) -> Self {
        Srgba::new(
            self.red + other.red,
            self.green + other.green,
            self.blue + other.blue,
            self.alpha + other.alpha,
        )
    }

    fn sub(&self, other: &Self) -> Self {
        Srgba::new(
            self.red - other.red,
            self.green - other.green,
            self.blue - other.blue,
            self.alpha - other.alpha,
        )
    }

    fn scale(&self, factor: f32) -> Self {
        Srgba::new(
            self.red * factor,
            self.green * factor,
            self.blue * factor,
            self.alpha * factor,
        )
    }

    fn mix(&self, other: &Self, factor: f32) -> Self {
        let this: Oklaba = (*self).into_color();
        let other: Oklaba = (*other).into_color();
        let mixed = vide::palette::Mix::mix(this.premultiply(), other.premultiply(), factor);
        Srgba::from_color(mixed.unpremultiply())
    }
}

impl Mixable for Quad {
    fn add(&self, other: &Self) -> Self {
        Quad {
            region: Rect::new(
                self.region.origin + other.region.origin.into(),
                self.region.size + other.region.size,
            ),
            color: self.color + other.color,
            corner_radius: self.corner_radius + other.corner_radius,
            edge_blur: self.edge_blur + other.edge_blur,
        }
    }

    fn sub(&self, other: &Self) -> Self {
        Quad {
            region: Rect::new(
                self.region.origin - other.region.origin,
                self.region.size - other.region.size,
            ),
            color: self.color - other.color,
            corner_radius: self.corner_radius - other.corner_radius,
            edge_blur: self.edge_blur - other.edge_blur,
        }
    }

    fn scale(&self, factor: f32) -> Self {
        Quad {
            region: Rect::new(
                (self.region.origin.to_vector() * factor).to_point(),
                self.region.size * factor,
            ),
            color: self.color * factor,
            corner_radius: self.corner_radius * factor,
            edge_blur: self.edge_blur * factor,
        }
    }
}
