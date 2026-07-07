use {bitflags::bitflags, wgpu::Backends};

bitflags! {
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Backend: u32 {
        const VULKAN = 1 << 0;
        const METAL = 1 << 1;
        const OPENGL = 1 << 2;
        const DX12 = 1 << 3;
        const WEBGPU = 1 << 4;

        const AUTO = Self::VULKAN.bits()
                    | Self::METAL.bits()
                    | Self::OPENGL.bits()
                    | Self::DX12.bits()
                    | Self::WEBGPU.bits();
    }
}

impl From<Backend> for Backends {
    fn from(value: Backend) -> Self {
        let mut backends = Backends::empty();

        if value.contains(Backend::VULKAN) {
            backends |= Backends::VULKAN;
        }
        if value.contains(Backend::METAL) {
            backends |= Backends::METAL;
        }
        if value.contains(Backend::OPENGL) {
            backends |= Backends::GL;
        }
        if value.contains(Backend::DX12) {
            backends |= Backends::DX12;
        }
        if value.contains(Backend::WEBGPU) {
            backends |= Backends::BROWSER_WEBGPU;
        }

        backends
    }
}
