pub const VERTEX_SHADER: &str = r#"
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) normal: vec3<f32>,
};

struct Uniforms {
    mvp: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) world_pos: vec3<f32>,
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.position = uniforms.mvp * vec4<f32>(input.position, 1.0);
    output.color = input.color;
    output.normal = input.normal;
    output.world_pos = input.position;
    return output;
}
"#;

pub const FRAGMENT_SHADER: &str = r#"
struct FragmentInput {
    @location(0) color: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) world_pos: vec3<f32>,
};

@fragment
fn fs_main(input: FragmentInput) -> @location(0) vec4<f32> {
    // направление на источник света
    let light_dir = normalize(vec3<f32>(0.3, 1.0, 0.4));

    // нормаль
    let n = normalize(input.normal);

    // ламберт
    let diff = max(dot(n, light_dir), 0.0);

    // амбиент
    let ambient = 0.2;

    // итог
    let lit = input.color * (ambient + 0.8 * diff);

    return vec4<f32>(lit, 1.0);
}
"#;
