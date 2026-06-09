struct ModelUniform {
   model: mat4x4<f32>
};

struct CameraUniform {
   view_projection: mat4x4<f32>
};

@group(0) @binding(0)
var<uniform> camera_uniform: CameraUniform;

@group(1) @binding(0)
var<uniform> model_uniform: ModelUniform;

@vertex
fn vs_main(@location(0) position: vec3<f32>) -> @builtin(position) vec4<f32> {
   return camera_uniform.view_projection * model_uniform.model * vec4<f32>(position, 1.0);
}
