use std::error::Error;
use wgpu::RenderPass;

use super::render_engine::RenderingEngineGPU;

/// Trait for a module that can render to an existing render pass.
/// Things rendered to the screen will possibly interact with other previously
/// rendered things.
pub trait RenderModule {
    fn prepare_to_render(&mut self, engine: &RenderingEngineGPU);

    fn commit_render<'pass>(&'pass self, render_pass: &mut RenderPass<'pass>) -> Result<(), Box<dyn Error>>;
}