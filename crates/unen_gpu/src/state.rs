pub struct GpuState {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

impl GpuState {
    pub async fn from_surface_and_instance(
        surface: &wgpu::Surface<'_>,
        instance: &wgpu::Instance,
    ) -> Self {
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(surface),
            })
            .await
            .expect("Failed to find valid adapter");

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("Main Device"),
                required_features: wgpu::Features::empty(),
                required_limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::defaults()
                },
                trace: wgpu::Trace::Off,
                ..Default::default()
            })
            .await
            .expect("Failed to create device and queue");

        Self { device, queue }
    }
}
