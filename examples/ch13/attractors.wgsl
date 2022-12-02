// vertex shader

struct VertexUniforms { 
    screenDimensions: vec2<f32>,
    particleSize: f32,
    _pad0: f32
}; 
@binding(0) @group(0) var<uniform> uniforms: VertexUniforms;

struct Input { 
    @location(0) vertexPosition: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) position: vec4<f32>,
};

struct Output { 
    @builtin(position) Position: vec4<f32>,
    @location(0) vColor: vec4<f32>,
};

@vertex
fn vs_main(input: Input) -> Output {
    var output: Output;
    output.vColor = input.color;
    output.Position = vec4<f32>(
        input.vertexPosition * uniforms.particleSize / uniforms.screenDimensions + input.position.xy,
        input.position.z,
        1.0
    );
    return output;
}

// fragment shader

@fragment
fn fs_main(@location(0) vColor: vec4<f32>) -> @location(0) vec4<f32> {
    return vec4<f32>(vColor.rgb * vColor.a, vColor.a);
}

// compute shader

struct PositionVelocity { 
    pv: array<vec4<f32>>, 
};

struct Mass { 
    mass1Position: vec4<f32>,
    mass2Position: vec4<f32>,
    mass3Position: vec4<f32>,
    mass1Factor: f32,
    mass2Factor: f32,
    mass3Factor: f32,
    _pado: f32,
};

@binding(0) @group(0) var<storage, read> positionIn: PositionVelocity;
@binding(1) @group(0) var<storage, read> velocityIn: PositionVelocity;
@binding(2) @group(0) var<storage, read_write> positionOut: PositionVelocity;
@binding(3) @group(0) var<storage, read_write> velocityOut: PositionVelocity;
@binding(4) @group(0) var<uniform> mass: Mass;

@compute
@workgroup_size(64)
fn cs_main(@builtin(global_invocation_id) GlobalInvocationID : vec3<u32>) {
    var index: u32 = GlobalInvocationID.x;
    var position: vec3<f32> = positionIn.pv[index].xyz;
    var velocity: vec3<f32> = velocityIn.pv[index].xyz;

    var massVec: vec3<f32> = mass.mass1Position.xyz - position;
    var massDist2: f32 = max(0.01, dot(massVec, massVec));
    var acceleration: vec3<f32> = mass.mass1Factor / massDist2 * normalize(massVec);

    massVec = mass.mass2Position.xyz-position; 
    massDist2 = max(0.01, dot(massVec, massVec)); 
    acceleration = acceleration + mass.mass2Factor/massDist2 * normalize(massVec); 

    massVec = mass.mass3Position.xyz-position; 
    massDist2 = max(0.01, dot(massVec, massVec)); 
    acceleration = acceleration + mass.mass3Factor/massDist2 * normalize(massVec);

    velocity = velocity + acceleration;
    velocity = 0.995 * velocity;

    // write back
    positionOut.pv[index] = vec4<f32>(position + velocity, 1.0);
    velocityOut.pv[index] = vec4<f32>(velocity, 1.0);
}
