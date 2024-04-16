use glyphon::{
    FontSystem, Resolution, SwashCache, TextArea, TextAtlas, TextBounds,
    TextRenderer as GTextRenderer, Weight,
};

const TEST_FONT: &[u8; 273900] = include_bytes!("../../../assets/fonts/JetBrainsMono-Regular.ttf");
const TEST_FONT2: &[u8; 15920] = include_bytes!("../../../assets/fonts/Nayten Sans.ttf");

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
        font_system.db_mut().load_font_data(TEST_FONT2.into());

        font_system.db_mut().set_monospace_family("JetBrains Mono");
        font_system.db_mut().set_sans_serif_family("Nayten Sans");

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
        queue: &wgpu::Queue,
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        clip_width: u32, clip_height: u32
    ) -> Result<(), glyphon::PrepareError> {
        let buffer = &self.get_test_text_buffer("Hi", (0.0, 0.0, clip_width as f32, clip_height as f32));

        let area = TextArea {
            buffer,
            left: 0.0,
            top: 0.0,
            scale: 1.0,
            bounds: TextBounds {
                left: 0,
                top: 0,
                right: clip_width as i32,
                bottom: clip_height as i32,
            },
            default_color: glyphon::Color::rgb(0xFF, 0xFF, 0xFF),
        };

        return self.gtext_renderer.prepare(
            device,
            queue,
            &mut self.font_system,
            &mut self.atlas,
            Resolution {
                width: config.width,
                height: config.height,
            },
            vec![area],
            &mut self.cache,
        );
    }

    pub fn get_test_text_buffer(
        &mut self,
        text: &str,
        aabb: (f32, f32, f32, f32),
    ) -> glyphon::Buffer {
        let mut bufferw =
            glyphon::Buffer::new(&mut self.font_system, glyphon::Metrics::new(8.0, 8.0));

        let attrs_normal = glyphon::Attrs::new()
            .family(glyphon::Family::Monospace)
            .color(glyphon::Color::rgb(0xDD, 0xDD, 0xDD));

        bufferw.set_size(&mut self.font_system, aabb.2, aabb.3);
        bufferw.set_rich_text(
            &mut self.font_system,
            [
                (
                    "This is an example of font rendering with ",
                    attrs_normal
                ),
                (
                    "JetBrains Mono",
                    glyphon::Attrs::new()
                        .family(glyphon::Family::Monospace)
                        .color(glyphon::Color::rgb(0x00, 0xAA, 0xFF)),
                ),
                (
                    " which looks absolutely ",
                    attrs_normal
                ),
                (
                    "✨gorgeous✨",
                    glyphon::Attrs::new()
                        .family(glyphon::Family::Monospace)
                        .style(glyphon::Style::Italic)
                        .color(glyphon::Color::rgb(250, 230, 60)),
                ),
                (
                    ".\n\n* And here is the lovely Nayten Sans (by benichi).\n\nنص مصمم لتخويف الأشخاص المتحيزين.",
                    attrs_normal
                ),
            ],
            glyphon::Shaping::Advanced,
        );
        bufferw.set_wrap(&mut self.font_system, glyphon::Wrap::Word);
        bufferw.shape_until_scroll(&mut self.font_system);

        let mut buffer =
            glyphon::Buffer::new(&mut self.font_system, glyphon::Metrics::new(16.0, 16.0));

        buffer.set_size(&mut self.font_system, 640.0, 360.0);
        buffer.set_text(
            &mut self.font_system,
            "Isn't is such a beautiful night?",
            glyphon::Attrs::new()
                .family(glyphon::Family::SansSerif)
                .weight(Weight::BOLD)
                .color(glyphon::Color::rgb(50, 50, 50)),
            glyphon::Shaping::Advanced,
        );
        buffer.set_wrap(&mut self.font_system, glyphon::Wrap::Word);
        buffer.shape_until_scroll(&mut self.font_system);

        return bufferw;
    }

    pub fn render<'pass>(
        &'pass self,
        render_pass: &mut wgpu::RenderPass<'pass>,
    ) -> Result<(), glyphon::RenderError> {
        self.gtext_renderer.render(&self.atlas, render_pass)
    }
}
