use glyphon::{
    Attrs, Color, FontSystem, Metrics, Resolution, SwashCache, TextArea, TextAtlas, TextBounds,
    TextRenderer as GTextRenderer, Weight,
};

const TEST_FONT: &[u8; 647344] = include_bytes!("../../assets/fonts/SourceSans.ttf");

pub struct TextRenderer {
    gtext_renderer: GTextRenderer,
    cache: SwashCache,
    atlas: TextAtlas,
    font_system: FontSystem,
}

impl TextRenderer {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        swapchain_format: wgpu::TextureFormat,
    ) -> Self {
        let mut font_system = FontSystem::new();

        font_system.db_mut().load_font_data(TEST_FONT.into());
        font_system.db_mut().set_sans_serif_family("Source Sans 3");

        let cache = SwashCache::new();
        let mut atlas: TextAtlas = TextAtlas::new(device, queue, swapchain_format);
        let text_renderer =
            GTextRenderer::new(&mut atlas, device, wgpu::MultisampleState::default(), None);

        Self {
            gtext_renderer: text_renderer,
            atlas,
            cache,
            font_system,
        }
    }

    pub fn prepare(
        &mut self,
        window: &winit::window::Window,
        queue: &wgpu::Queue,
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
    ) {
        let size = window.inner_size();
        let font_size = 16.0;
        let line_height = 1.2 * font_size;

        let mut buffer =
            glyphon::Buffer::new(&mut self.font_system, Metrics::new(font_size, line_height));

        buffer.set_size(&mut self.font_system, size.width as f32, size.height as f32);
        buffer.set_text(
            &mut self.font_system,
            "Lorem ipsum.",
            Attrs::new()
                .family(glyphon::Family::SansSerif)
                .weight(Weight::BOLD)
                .color(Color::rgb(50, 50, 50)),
            glyphon::Shaping::Advanced,
        );
        buffer.set_wrap(&mut self.font_system, glyphon::Wrap::Word);
        buffer.shape_until_scroll(&mut self.font_system);

        let text_areas = [TextArea {
            buffer: &buffer,
            left: 0.0,
            top: 0.0,
            scale: 1.0,
            bounds: TextBounds {
                left: 0,
                top: 0,
                right: 640,
                bottom: 360,
            },
            default_color: Color::rgb(255, 255, 255),
        }];

        let _ = self.gtext_renderer.prepare(
            device,
            queue,
            &mut self.font_system,
            &mut self.atlas,
            Resolution {
                width: config.width,
                height: config.height,
            },
            text_areas,
            &mut self.cache,
        );
    }

    pub fn render<'a>(
        &'a self,
        render_pass: &mut wgpu::RenderPass<'a>,
    ) -> Result<(), glyphon::RenderError> {
        self.gtext_renderer.render(&self.atlas, render_pass)
    }
}
