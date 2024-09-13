use glamour::prelude::*;
use lazy_static::lazy_static;
use palette::{FromColor, IntoColor, Oklaba, Srgba};
use vide::Quad;

fn hex_to_srgba(hex: &str) -> Srgba {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap() as f32 / 255.;
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap() as f32 / 255.;
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap() as f32 / 255.;
    let a = if hex.len() == 8 {
        u8::from_str_radix(&hex[6..8], 16).unwrap() as f32 / 255.
    } else {
        1.
    };
    Srgba::new(r, g, b, a)
}

lazy_static! {
    pub static ref BACKGROUND_DIM: Srgba = hex_to_srgba("#1e2326");
    pub static ref BACKGROUND0: Srgba = hex_to_srgba("#272e33");
    pub static ref BACKGROUND1: Srgba = hex_to_srgba("#2e383c");
    pub static ref BACKGROUND2: Srgba = hex_to_srgba("#374145");
    pub static ref BACKGROUND3: Srgba = hex_to_srgba("#414b50");
    pub static ref BACKGROUND4: Srgba = hex_to_srgba("#495156");
    pub static ref BACKGROUND5: Srgba = hex_to_srgba("#4f5b58");
    pub static ref BACKGROUND_RED: Srgba = hex_to_srgba("#4c3743");
    pub static ref BACKGROUND_VISUAL: Srgba = hex_to_srgba("#493B40");
    pub static ref BACKGROUND_YELLOW: Srgba = hex_to_srgba("#45443C");
    pub static ref BACKGROUND_GREEN: Srgba = hex_to_srgba("#3C4841");
    pub static ref BACKGROUND_BLUE: Srgba = hex_to_srgba("#384B55");
    pub static ref RED: Srgba = hex_to_srgba("#e67e80");
    pub static ref ORANGE: Srgba = hex_to_srgba("#e69875");
    pub static ref YELLOW: Srgba = hex_to_srgba("#dbbc7f");
    pub static ref GREEN: Srgba = hex_to_srgba("#a7c080");
    pub static ref BLUE: Srgba = hex_to_srgba("#7fbbb4");
    pub static ref AQUA: Srgba = hex_to_srgba("#83C092");
    pub static ref PURPLE: Srgba = hex_to_srgba("#d699b6");
    pub static ref FOREGROUND: Srgba = hex_to_srgba("#d3c6aa");
    pub static ref STATUSLINE_1: Srgba = hex_to_srgba("#a7c080");
    pub static ref STATUSLINE_2: Srgba = hex_to_srgba("#d3c6aa");
    pub static ref STATUSLINE_3: Srgba = hex_to_srgba("#e67e80");
    pub static ref GRAY_0: Srgba = hex_to_srgba("#7a8478");
    pub static ref GRAY_1: Srgba = hex_to_srgba("#859289");
    pub static ref GRAY_2: Srgba = hex_to_srgba("#9da9a0");
    pub static ref CLOSE: Srgba = hex_to_srgba("#c42b1c");
}

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
        let mixed = palette::Mix::mix(this.premultiply(), other.premultiply(), factor);
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
