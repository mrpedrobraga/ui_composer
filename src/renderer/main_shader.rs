pub fn get_main_shader() -> wgpu::ShaderModuleDescriptor<'static> {
    wgpu::ShaderModuleDescriptor {
        label: Some("Main Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("./main.wgsl").into()),
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ProgramUniforms {
    pub window_size: [f32; 4],
}
