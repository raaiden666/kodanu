struct MaterialUniform {
    base_color: vec4<f32>,
}

@group(2) @binding(0)
var<uniform> material: MaterialUniform;

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return material.base_color;
}
