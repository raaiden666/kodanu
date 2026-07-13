mod camera;
mod config;
mod gpu;
mod material;
mod mesh;
mod model;
mod pipeline;
mod renderer;
mod resources;
mod setup;
mod shader;

pub use config::{Backend, RendererConfig, SampleCount};
pub use renderer::{FrameStatus, RenderItem, Renderer};

pub(crate) use camera::{CameraRenderer, CameraUniform};
pub(crate) use material::{GpuMaterial, MaterialUniform};
pub(crate) use model::{MaterialCache, ModelSrorageBuffer, ModelUniform};
pub(crate) use resources::AssetResources;
pub(crate) use shader::{fragment_shader, vertex_shader};
