struct VSOutput {
    @location(0) v2f_color: vec4<f32>,
    @builtin(position) position: vec4<f32>,
};

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32
) -> VSOutput {
    var pos = array<vec2<f32>, 3>(
        vec2<f32>( 0.0,  0.5),
        vec2<f32>(-0.5, -0.5),
        vec2<f32>( 0.5, -0.5)
    );
    var color = array<vec3<f32>,3>( 
        vec3<f32>(1.0, 0.0, 0.0), 
        vec3<f32>(0.0, 1.0, 0.0), 
        vec3<f32>(0.0, 0.0, 1.0) 
    );

    var out: VSOutput;
    out.position = vec4<f32>(pos[in_vertex_index], 0.0, 1.0); 
    out.v2f_color = vec4<f32>(color[in_vertex_index], 1.0); 
    return out;
}

@fragment
fn fs_main(in: VSOutput) -> @location(0) vec4<f32> {
    return in.v2f_color;
}
