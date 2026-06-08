struct VertexInput {
    @location(0) position: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
};

struct ModelUniform {
   model: mat4x4<f32>
};

@group(0) @binding(0)
var<uniform> model_uniform: ModelUniform;

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
   var output: VertexOutput;

   output.position = model_uniform.model * vec4<f32>(input.position, 1.0);

   return output;
}
