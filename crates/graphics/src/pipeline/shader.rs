use wgpu::{Device, ShaderModule, ShaderModuleDescriptor, ShaderSource};

pub struct Shader {
    module: ShaderModule,
}

impl Shader {
    pub fn from_wgsl(device: &Device, label: &str, source: &str) -> Self {
        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some(label),
            source: ShaderSource::Wgsl(source.into()),
        });

        Self { module: shader }
    }

    pub fn module(&self) -> &ShaderModule {
        &self.module
    }
}
