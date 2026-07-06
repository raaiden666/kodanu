use wgpu::{Device, ShaderModule, ShaderModuleDescriptor, ShaderSource};

pub fn vertex_shader(device: &Device) -> ShaderModule {
    device.create_shader_module(ShaderModuleDescriptor {
        label: Some("Mesh VS"),
        source: ShaderSource::Wgsl(
            include_str!("../../../../resources/shaders/wgsl/mesh.vert.wgsl").into(),
        ),
    })
}

pub fn fragment_shader(device: &Device) -> ShaderModule {
    device.create_shader_module(ShaderModuleDescriptor {
        label: Some("Mesh FS"),
        source: ShaderSource::Wgsl(
            include_str!("../../../../resources/shaders/wgsl/mesh.frag.wgsl").into(),
        ),
    })
}
