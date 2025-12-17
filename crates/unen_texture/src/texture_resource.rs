use unen_resource::prelude::Resource;

pub struct TextureResource {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub size: wgpu::Extent3d,
}

impl Resource for TextureResource {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
