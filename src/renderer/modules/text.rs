use glyphon::{
    FontSystem, Resolution, SwashCache, TextArea, TextAtlas, TextBounds,
    TextRenderer as GTextRenderer, Weight,
};

use crate::renderer::engine::{render_engine::RenderingEngineGPU, render_module::RenderModule};

const TEST_FONT: &[u8; 273900] = include_bytes!("../../../assets/fonts/JetBrainsMono-Regular.ttf");
const TEST_FONT2: &[u8; 15920] = include_bytes!("../../../assets/fonts/Nayten Sans.ttf");

pub struct TextRenderModule {
    gtext_renderer: GTextRenderer,
    cache: SwashCache,
    atlas: TextAtlas,
    font_system: FontSystem,
}

impl TextRenderModule {
    pub fn new(
        gpu: &RenderingEngineGPU
    ) -> Self {
        let mut font_system = FontSystem::new();

        font_system.db_mut().load_font_data(TEST_FONT.into());
        font_system.db_mut().load_font_data(TEST_FONT2.into());

        font_system.db_mut().set_monospace_family("JetBrains Mono");
        font_system.db_mut().set_sans_serif_family("Nayten Sans");

        let cache = SwashCache::new();
        let mut atlas: TextAtlas = TextAtlas::new(&gpu.device, &gpu.queue, gpu.surface_config.format);

        let text_renderer =
            GTextRenderer::new(
                &mut atlas,
                &gpu.device,
                wgpu::MultisampleState::default(),
                None
            );

        Self {
            gtext_renderer: text_renderer,
            atlas,
            cache,
            font_system,
        }
    }

    pub fn prepare_text_areas(
        &mut self,
        queue: &wgpu::Queue,
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        window_width: u32, window_height: u32
    ) -> Result<(), glyphon::PrepareError> {
        let buffer = &self.get_test_text_buffer("Hi", (0.0, 0.0, window_width as f32, window_height as f32));

        let area = TextArea {
            buffer,
            left: 0.0,
            top: 0.0,
            scale: 1.0,
            bounds: TextBounds {
                left: 0,
                top: 0,
                right: window_width as i32,
                bottom: window_height as i32,
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
            glyphon::Buffer::new(&mut self.font_system, glyphon::Metrics::new(12.0, 12.0));

        let attrs_normal = glyphon::Attrs::new()
            .family(glyphon::Family::Monospace)
            .color(glyphon::Color::rgb(0xDD, 0xDD, 0xDD));

        bufferw.set_size(&mut self.font_system, aabb.2, aabb.3);
        bufferw.set_rich_text(
            &mut self.font_system,
            [(
                "This is a contrived example of UI Composer's capabilities regarding text rendering.\n\nSo, it uses Glyphon under the hood for text rendering (which uses cosmic-text for shaping, etagere for atlas-ing and wgpu middleware for rendering).", attrs_normal
            )],
            glyphon::Shaping::Basic
        );
        bufferw.set_wrap(&mut self.font_system, glyphon::Wrap::Word);
        bufferw.shape_until_scroll(&mut self.font_system);

        return bufferw;
    }
}

impl RenderModule for TextRenderModule {
    fn prepare(&mut self, engine: &RenderingEngineGPU) {
        self.prepare_text_areas(&engine.queue, &engine.device, &engine.surface_config, engine.window_size.width, engine.window_size.height);
    }

    fn render<'pass>(
        &'pass self,
        render_pass: &mut wgpu::RenderPass<'pass>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.gtext_renderer.render(&self.atlas, render_pass)?;
        Ok(())
    }
}
