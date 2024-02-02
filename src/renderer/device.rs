use std::error::Error;

pub fn create_instance() -> wgpu::Instance {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    });
    instance
}

pub async fn get_device(
    adapter: &wgpu::Adapter,
) -> Result<(wgpu::Device, wgpu::Queue), Box<dyn Error>> {
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
                label: None,
            },
            None,
        )
        .await?;
    Ok((device, queue))
}

pub async fn get_adapter(instance: wgpu::Instance, surface: &wgpu::Surface) -> wgpu::Adapter {
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptionsBase {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(surface),
        })
        .await;
    let adapter = match adapter {
        Some(v) => v,
        None => todo!(),
    };
    adapter
}

pub fn create_surface(
    instance: &wgpu::Instance,
    window: &winit::window::Window,
) -> Result<wgpu::Surface, Box<dyn Error>> {
    let surface = unsafe { instance.create_surface(window) }?;
    Ok(surface)
}

pub fn get_default_surface_configuration(
    surface_format: wgpu::TextureFormat,
    window_size: winit::dpi::PhysicalSize<u32>,
    surface_capabilities: wgpu::SurfaceCapabilities,
) -> wgpu::SurfaceConfiguration {
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        // INFO: `width` and `height` can never be 0, otherwise the program
        // might crash unexpectedly.
        width: window_size.width,
        height: window_size.height,
        // TODO: This will be choosable by the user futurely.
        present_mode: surface_capabilities.present_modes[0],
        alpha_mode: surface_capabilities.alpha_modes[0],
        view_formats: Vec::new(),
    };
    config
}

pub fn get_surface_format(surface_capabilities: &wgpu::SurfaceCapabilities) -> wgpu::TextureFormat {
    let surface_format = surface_capabilities
        .formats
        .iter()
        .copied()
        .filter(|f| f.is_srgb())
        .next()
        .unwrap_or(surface_capabilities.formats[0]);
    surface_format
}
