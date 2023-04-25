#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod widgets;
mod controllers;
mod persistence;
mod app_data;

use druid::{
    AppLauncher, LocalizedString, WindowDesc, WidgetExt, Color, theme,
};

use app_data::AppData;
use controllers::*;
use persistence::read_or;
use widgets::{
    flow::Flow,
    todo::todo,
    dot_grid::dot_grid
};

const FOREGROUND: Color = Color::rgb8(251, 187, 173);
const ACCENT_PINK: Color = Color::rgb8(238, 134, 149);
const ACCENT_BLUE: Color = Color::rgb8(74, 122, 150);
const ACCENT_DARK_BLUE: Color = Color::rgb8(51, 63, 88);
const BACKGROUND: Color = Color::rgb8(41, 40, 49);

fn main() {
    let window = WindowDesc::new(|| Flow::new(|| todo())
            .background(dot_grid())
            .draggable(true)
            .undo_root()
            .selection_root()
    ).title(LocalizedString::new("Pando"));
    AppLauncher::with_window(window)
        .configure_env(|env, _| {
            env.set(theme::FOREGROUND_LIGHT, FOREGROUND);
            env.set(theme::FOREGROUND_DARK, ACCENT_PINK);
            env.set(theme::BACKGROUND_LIGHT, ACCENT_DARK_BLUE);
            env.set(theme::BACKGROUND_DARK, BACKGROUND);
            env.set(theme::PRIMARY_LIGHT, FOREGROUND);
            env.set(theme::PRIMARY_DARK, BACKGROUND);
            env.set(theme::BORDER_DARK, ACCENT_DARK_BLUE);
            env.set(theme::BORDER_LIGHT, ACCENT_BLUE);
            env.set(theme::LABEL_COLOR, FOREGROUND);
        })
        .launch(read_or(AppData::new()))
        .expect("Launch Failed");
}
