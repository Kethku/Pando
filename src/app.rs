use glamour::prelude::*;
use palette::Srgba;
use vide::prelude::*;

use crate::{
    framework::{
        components::{background::Background, window_buttons::WindowButtons},
        context::{Context, DrawContext},
        runner::FrameworkApplication,
    },
    util::*,
};

pub struct App {
    pub background: Background,
    pub window_buttons: WindowButtons,
}

impl App {
    pub fn new() -> App {
        App {
            background: Background::new(point!(0., 0.)),
            window_buttons: WindowButtons::new(),
        }
    }
}

impl FrameworkApplication for App {
    fn update(&mut self, cx: &Context) -> bool {
        let mut should_draw = false;

        should_draw |= self.background.update(cx);
        should_draw |= self.window_buttons.update(cx);

        should_draw
    }

    fn draw(&self, cx: &mut DrawContext) {
        self.background.draw(cx);
        let offset = self.background.offset();
        cx.add_layer(
            Layer::new()
                .with_quad(
                    Quad::new(
                        Rect::new(point2!(50., 52.5) + offset.to_vector(), size!(100., 150.)),
                        Srgba::new(0., 0., 0., 0.8),
                    )
                    .with_edge_blur(10.)
                    .with_corner_radius(10.),
                )
                .with_quad(
                    Quad::new(
                        Rect::new(point2!(50., 50.) + offset.to_vector(), size!(100., 150.)),
                        *BACKGROUND1,
                    )
                    .with_corner_radius(10.),
                ),
        );
        self.window_buttons.draw(cx);
    }
}
