use crate::{pipeline::GraphicsPipeline, resources::Mesh};

use wgpu::RenderPass;

pub struct MeshRenderer<'a> {
    pipeline: &'a GraphicsPipeline,
    mesh: &'a Mesh,
}

impl<'a> MeshRenderer<'a> {
    pub fn new(pipeline: &'a GraphicsPipeline, mesh: &'a Mesh) -> Self {
        Self { pipeline, mesh }
    }

    pub fn draw<'pass>(&self, pass: &mut RenderPass<'pass>) {
        pass.set_pipeline(self.pipeline.raw());
        pass.set_vertex_buffer(0, self.mesh.vertex_buffer().slice(..));
        pass.draw(0..self.mesh.vertex_count(), 0..1);
    }
}
