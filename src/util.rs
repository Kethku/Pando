#![allow(dead_code)]

use std::sync::LazyLock;

use aspen::vide::prelude::*;

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

pub static BACKGROUND_DIM: LazyLock<Srgba> = LazyLock::new(|| hex_to_srgba("#1e2326"));
pub static BACKGROUND0: LazyLock<Srgba> = LazyLock::new(|| hex_to_srgba("#272e33"));
pub static BACKGROUND1: LazyLock<Srgba> = LazyLock::new(|| hex_to_srgba("#2e383c"));
pub static BACKGROUND2: LazyLock<Srgba> = LazyLock::new(|| hex_to_srgba("#374145"));
pub static BACKGROUND3: LazyLock<Srgba> = LazyLock::new(|| hex_to_srgba("#414b50"));
pub static BACKGROUND4: LazyLock<Srgba> = LazyLock::new(|| hex_to_srgba("#495156"));
pub static BACKGROUND5: LazyLock<Srgba> = LazyLock::new(|| hex_to_srgba("#4f5b58"));
pub static BACKGROUND_RED: LazyLock<Srgba> = LazyLock::new(|| hex_to_srgba("#4c3743"));
pub static BACKGROUND_VISUAL: LazyLock<Srgba> = LazyLock::new(|| hex_to_srgba("#493B40"));
pub static BACKGROUND_YELLOW: LazyLock<Srgba> = LazyLock::new(|| hex_to_srgba("#45443C"));
pub static BACKGROUND_GREEN: LazyLock<Srgba> = LazyLock::new(|| hex_to_srgba("#3C4841"));
pub static BACKGROUND_BLUE: LazyLock<Srgba> = LazyLock::new(|| hex_to_srgba("#384B55"));
pub static RED: LazyLock<Srgba> = LazyLock::new(|| hex_to_srgba("#e67e80"));
pub static ORANGE: LazyLock<Srgba> = LazyLock::new(|| hex_to_srgba("#e69875"));
pub static YELLOW: LazyLock<Srgba> = LazyLock::new(|| hex_to_srgba("#dbbc7f"));
pub static GREEN: LazyLock<Srgba> = LazyLock::new(|| hex_to_srgba("#a7c080"));
pub static BLUE: LazyLock<Srgba> = LazyLock::new(|| hex_to_srgba("#7fbbb4"));
pub static AQUA: LazyLock<Srgba> = LazyLock::new(|| hex_to_srgba("#83C092"));
pub static PURPLE: LazyLock<Srgba> = LazyLock::new(|| hex_to_srgba("#d699b6"));
pub static FOREGROUND: LazyLock<Srgba> = LazyLock::new(|| hex_to_srgba("#d3c6aa"));
pub static STATUSLINE_1: LazyLock<Srgba> = LazyLock::new(|| hex_to_srgba("#a7c080"));
pub static STATUSLINE_2: LazyLock<Srgba> = LazyLock::new(|| hex_to_srgba("#d3c6aa"));
pub static STATUSLINE_3: LazyLock<Srgba> = LazyLock::new(|| hex_to_srgba("#e67e80"));
pub static GRAY_0: LazyLock<Srgba> = LazyLock::new(|| hex_to_srgba("#7a8478"));
pub static GRAY_1: LazyLock<Srgba> = LazyLock::new(|| hex_to_srgba("#859289"));
pub static GRAY_2: LazyLock<Srgba> = LazyLock::new(|| hex_to_srgba("#9da9a0"));
pub static CLOSE: LazyLock<Srgba> = LazyLock::new(|| hex_to_srgba("#c42b1c"));
