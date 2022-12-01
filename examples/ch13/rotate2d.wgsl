struct Data {
    numbers: array<f32>,
};

struct AngleData {
    angle: f32,
};

@group(0) @binding(0) var<storage, read>       point_data : Data;
@group(0) @binding(1) var<uniform>             angle_data : AngleData;
@group(0) @binding(2) var<storage, read_write> result     : Data;

@compute
@workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    var index: u32 = global_id.x;
    var pt: vec2<f32> = normalize(vec2<f32>(point_data.numbers[0], point_data.numbers[1]));
    var p0: f32 = pt[0];
    var p1: f32 = pt[1];
    var theta: f32 = angle_data.angle*3.1415926/180.0;
    var res: f32 = 0.0;
    if (index == 0u) {
        res = p0 * cos(theta) - p1 * sin(theta);
    } else {
        res = p0 * sin(theta) + p1 * cos(theta);
    }
    result.numbers[index] = res;
}
