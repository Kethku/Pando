use std::{num::NonZeroUsize, sync::Arc};

use vello::{
    peniko::color::palette::css::BLACK,
    util::{RenderContext, RenderSurface},
    wgpu::*,
    AaConfig, Renderer, RendererOptions, Scene,
};
use winit::window::Window;

pub struct WinitRenderer {
    pub render_context: RenderContext,
    pub render_surface: RenderSurface<'static>,
    pub width: u32,
    pub height: u32,
    pub window: Arc<Window>,
    renderer: Renderer,
}

impl WinitRenderer {
    // Creating some of the wgpu types requires async code
    pub async fn new(window: Arc<Window>) -> Self {
        let mut render_context = RenderContext::new();

        let size = window.inner_size();
        let render_surface = render_context
            .create_surface(
                window.clone(),
                size.width,
                size.height,
                PresentMode::Immediate,
            )
            .await
            .unwrap();

        #[cfg(target_os = "macos")]
        window.request_redraw();

        let device_index = render_context
            .device(Some(&render_surface.surface))
            .await
            .unwrap();

        let renderer = Renderer::new(
            &render_context.devices[device_index].device,
            RendererOptions {
                use_cpu: false,
                antialiasing_support: vello::AaSupport::area_only(),
                num_init_threads: NonZeroUsize::new(1),
                pipeline_cache: None,
            },
        )
        .unwrap();

        Self {
            render_context,
            render_surface,
            width: size.width,
            height: size.height,
            renderer,
            window,
        }
    }

    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        self.width = new_width;
        self.height = new_height;

        if new_width != 0 && new_height != 0 {
            self.render_context
                .resize_surface(&mut self.render_surface, new_width, new_height);
        }
    }

    pub fn resumed(&mut self) {
        self.render_surface = futures::executor::block_on(self.render_context.create_surface(
            self.window.clone(),
            self.width,
            self.height,
            PresentMode::Immediate,
        ))
        .unwrap();
        self.window.request_redraw();
    }

    pub fn draw(&mut self, scene: &Scene) -> bool {
        let device_handle = &self.render_context.devices[self.render_surface.dev_id];
        self.renderer
            .render_to_texture(
                &device_handle.device,
                &device_handle.queue,
                scene,
                &self.render_surface.target_view,
                &vello::RenderParams {
                    base_color: BLACK,
                    width: self.width,
                    height: self.height,
                    antialiasing_method: AaConfig::Area,
                },
            )
            .expect("Could not render to texture");

        let surface_texture = self
            .render_surface
            .surface
            .get_current_texture()
            .expect("failed to get surface texture");

        let mut encoder = device_handle
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Surface Blit"),
            });
        self.render_surface.blitter.copy(
            &device_handle.device,
            &mut encoder,
            &self.render_surface.target_view,
            &surface_texture
                .texture
                .create_view(&TextureViewDescriptor::default()),
        );
        device_handle.queue.submit([encoder.finish()]);
        surface_texture.present();

        true
    }
}
