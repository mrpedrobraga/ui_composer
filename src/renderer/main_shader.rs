pub fn get_main_shader() -> wgpu::ShaderModuleDescriptor<'static> {
    wgpu::ShaderModuleDescriptor {
        label: Some("Main Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("./main.wgsl").into()),
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ProgramUniforms {
    pub window_size: [[f32; 4]; 4],
    pub camera_position: [f32; 4],
}

impl Default for ProgramUniforms {
    fn default() -> Self {
        Self { window_size: Default::default(), camera_position: Default::default() }
    }
}