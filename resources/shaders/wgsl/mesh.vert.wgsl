struct ModelUniform {
   model: mat4x4<f32>
};

struct CameraUniform {
   view_projection: mat4x4<f32>
};

struct ModelStorage {
    models: array<ModelUniform>
}

@group(0) @binding(0)
var<uniform> camera_uniform: CameraUniform;

@group(1) @binding(0)
var<storage, read> model_storage: ModelStorage;

@vertex
fn vs_main(@location(0) position: vec3<f32>, @builtin(instance_index) instance_index: u32) -> @builtin(position) vec4<f32> {
   let model = model_storage.models[instance_index].model;

   return camera_uniform.view_projection * model * vec4<f32>(position, 1.0);
}
