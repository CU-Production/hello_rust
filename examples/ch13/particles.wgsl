// vertex shader

struct Transform { 
    projectionMatrix: mat4x4<f32>,
    viewMatrix: mat4x4<f32>,
};
@group(0) @binding(0) var<uniform> transform: Transform;

struct Input { 
    @location(0) position: vec2<f32>,
    @location(1) instancePosition: vec2<f32>,
    @location(2) instanceVelocity: vec2<f32>,
    @location(3) color: vec3<f32>,
    @location(4) scaleFactor: f32,
};

struct Output { 
    @builtin(position) Position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_main (input: Input) -> Output { 
    var output: Output; 
    let scaleMatrix = mat4x4<f32>(
        vec4<f32>(input.scaleFactor, 0.0,                             0.0, 0.0),
        vec4<f32>(0.0,               input.scaleFactor,               0.0, 0.0),
        vec4<f32>(0.0,               0.0,               input.scaleFactor, 0.0),
        vec4<f32>(0.0,               0.0,                             0.0, 1.0),
    );
    let pos = vec4<f32>(input.position, 0.0, 1.0);
    let ins_pos = vec4<f32>(input.instancePosition, 0.0, 1.0);
    let transformedPos = scaleMatrix *pos + ins_pos; 
    output.Position = transform.projectionMatrix * transform.viewMatrix * transformedPos;
    output.color = vec4<f32>(input.color, 1.0);
    return output;
}

// fragment shader 

@fragment
fn fs_main (in: Output) -> @location(0) vec4<f32> {
    return in.color;
}

// compute shader

struct ParticleData { 
    position: vec2<f32>,
    velocity: vec2<f32>,
    color: vec3<f32>,
    radius: f32,
};

struct ParticlesBuffer { 
    particles: array<ParticleData>,
}; 
@group(0) @binding(0) var<storage, read_write> particlesBuffer: ParticlesBuffer;

struct Uniforms { 
    canvasSize: vec2<f32>,
    deltaTime: f32,
    bounceFactor: f32,
    acceLeft: vec4<f32>,
    acceRight: vec4<f32>,
}; 
@group(0) @binding(1) var<uniform> uniforms: Uniforms;

@compute 
@workgroup_size(64, 1, 1) 
fn cs_main(@builtin(global_invocation_id) GlobalInvocationID : vec3<u32>) { 
    let index = GlobalInvocationID.x;
    let canvasWidth = uniforms.canvasSize.x * 2.0;
    let canvasHeight = uniforms.canvasSize.y * 2.0;

    let particleRadius = particlesBuffer.particles[index].radius;
    let vx = particlesBuffer.particles[index].velocity.x;
    let vy = particlesBuffer.particles[index].velocity.y;
    if (particlesBuffer.particles[index].position.x < canvasWidth * 0.5) {
        if (particlesBuffer.particles[index].position.y > canvasHeight * 0.5) {
            particlesBuffer.particles[index].velocity.x = vx + uniforms.acceLeft.x * uniforms.deltaTime;
            particlesBuffer.particles[index].velocity.y = vy + uniforms.acceLeft.y * uniforms.deltaTime;
        } else {
            particlesBuffer.particles[index].velocity.x = vx + uniforms.acceLeft.z * uniforms.deltaTime;
            particlesBuffer.particles[index].velocity.y = vy + uniforms.acceLeft.w * uniforms.deltaTime;
        }
    } else {
        if (particlesBuffer.particles[index].position.y > canvasHeight * 0.5) {
            particlesBuffer.particles[index].velocity.x = vx + uniforms.acceRight.x * uniforms.deltaTime;
            particlesBuffer.particles[index].velocity.y = vy + uniforms.acceRight.y * uniforms.deltaTime;
        } else {
            particlesBuffer.particles[index].velocity.x = vx + uniforms.acceRight.z * uniforms.deltaTime;
            particlesBuffer.particles[index].velocity.y = vy + uniforms.acceRight.w * uniforms.deltaTime;
        }
    }
    particlesBuffer.particles[index].position.x = particlesBuffer.particles[index].position.x
        + particlesBuffer.particles[index].velocity.x * uniforms.deltaTime;
    particlesBuffer.particles[index].position.y = particlesBuffer.particles[index].position.y
        + particlesBuffer.particles[index].velocity.y * uniforms.deltaTime;

    // handle screen viewport
    if (particlesBuffer.particles[index].position.x + particleRadius * 0.5 > canvasWidth) {
        particlesBuffer.particles[index].position.x = canvasWidth - particleRadius * 0.5;
        particlesBuffer.particles[index].velocity.x = vx * -uniforms.bounceFactor;
    } else if (particlesBuffer.particles[index].position.x - particleRadius * 0.5 < 0.0) {
        particlesBuffer.particles[index].position.x = particleRadius * 0.5;
        particlesBuffer.particles[index].velocity.x = vx * -uniforms.bounceFactor;
    }
    if (particlesBuffer.particles[index].position.y + particleRadius * 0.5 > canvasHeight) {
        particlesBuffer.particles[index].position.y = canvasHeight - particleRadius * 0.5;
        particlesBuffer.particles[index].velocity.y = vy * -uniforms.bounceFactor;
    } else if (particlesBuffer.particles[index].position.y - particleRadius * 0.5 < 0.0) {
        particlesBuffer.particles[index].position.y = particleRadius * 0.5;
        particlesBuffer.particles[index].velocity.y = vy * -uniforms.bounceFactor;
    }
} 
