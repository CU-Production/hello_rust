#![allow(dead_code)]
use cgmath::Matrix4;
use rand::{
    distributions::{Distribution, Uniform},
    SeedableRng,
};
use std::time::SystemTime;
use wgpu::util::DeviceExt;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::Window,
    window::WindowBuilder,
};
#[path = "../../common/transforms.rs"]
mod transforms;

const PARTICLES_PER_GROUP: u32 = 64;

struct State {
    init: transforms::InitWgpu,

    // compute
    particle_buffer: wgpu::Buffer,
    particle_uniform_data: Vec<f32>,
    particle_uniform_buffer: wgpu::Buffer,
    compute_bind_group: wgpu::BindGroup,
    compute_pipeline: wgpu::ComputePipeline,
    work_group_count: u32,
    num_particles: u32,

    // render
    view_mat: Matrix4<f32>,
    project_mat: Matrix4<f32>,
    uniform_buffer: wgpu::Buffer,
    vertex_buffer: wgpu::Buffer,
    render_bind_group: wgpu::BindGroup,
    render_pipeline: wgpu::RenderPipeline,

    // parameters
    unif_mp: Uniform<f32>,
    rng: rand::rngs::StdRng,
    start: SystemTime,
    t0: f32,
    t1: f32,
}

impl State {
    fn required_limits() -> wgpu::Limits {
        wgpu::Limits::downlevel_defaults()
    }

    fn required_downlevel_capabilities() -> wgpu::DownlevelCapabilities {
        wgpu::DownlevelCapabilities {
            flags: wgpu::DownlevelFlags::COMPUTE_SHADERS,
            ..Default::default()
        }
    }

    async fn new(window: &Window, num_particles: u32, particle_size: f32) -> Self {
        let start = SystemTime::now();
        let init = transforms::InitWgpu::init_wgpu(window).await;

        let shader = init
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("particles.wgsl").into()),
            });

        // compute

        let mut particle_data = vec![0.0f32; num_particles as usize * 8];
        let mut rng = rand::rngs::StdRng::seed_from_u64(42);
        let unif_mp = Uniform::new_inclusive(-1.0, 1.0);
        let unif_p = Uniform::new_inclusive(0.0, 1.0);
        for particle_chunck in particle_data.chunks_mut(8) {
            // position
            particle_chunck[0] = unif_p.sample(&mut rng) * init.config.width as f32 * 2.0;
            particle_chunck[1] = unif_p.sample(&mut rng) * init.config.height as f32 * 2.0;
            // velocity
            particle_chunck[2] = unif_mp.sample(&mut rng) * 400.0;
            particle_chunck[3] = unif_mp.sample(&mut rng) * 400.0;
            // color rgb
            particle_chunck[4] = unif_p.sample(&mut rng);
            particle_chunck[5] = unif_p.sample(&mut rng);
            particle_chunck[6] = unif_p.sample(&mut rng);
            // scale factor for particle size
            particle_chunck[7] = unif_p.sample(&mut rng) + 1.0;
        }

        let particle_buffer = init
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("Particle Buffer")),
                contents: bytemuck::cast_slice(&particle_data),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::STORAGE,
            });

        let particle_uniform_data = [
            init.config.width as f32, // size
            init.config.height as f32,
            0.0,                              // delta_frame
            0.5,                              // bounce_factor
            unif_mp.sample(&mut rng) * 240.0, // acceleration left
            unif_mp.sample(&mut rng) * 240.0,
            unif_mp.sample(&mut rng) * 240.0,
            unif_mp.sample(&mut rng) * 240.0,
            unif_mp.sample(&mut rng) * 240.0, // acceleration right
            unif_mp.sample(&mut rng) * 240.0,
            unif_mp.sample(&mut rng) * 240.0,
            unif_mp.sample(&mut rng) * 240.0,
        ]
        .to_vec();

        let particle_uniform_buffer =
            init.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(&format!("Particle Uniform Buffer")),
                    contents: bytemuck::cast_slice(&particle_uniform_data),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                });

        let compute_bind_group_layout =
            init.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Storage { read_only: false },
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                    ],
                    label: None,
                });

        let compute_bind_group = init.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &compute_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: particle_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: particle_uniform_buffer.as_entire_binding(),
                },
            ],
            label: Some("Compute Bind Group"),
        });

        let compute_pipeline_layout =
            init.device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("compute"),
                    bind_group_layouts: &[&compute_bind_group_layout],
                    push_constant_ranges: &[],
                });

        let compute_pipeline =
            init.device
                .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                    label: Some("Compute Pipeline"),
                    layout: Some(&compute_pipeline_layout),
                    module: &shader,
                    entry_point: "cs_main",
                });

        // render

        let camera_position = (0.0, 0.0, 2.0).into();
        let look_direction = (0.0, 0.0, 0.0).into();
        let up_direction = cgmath::Vector3::unit_y();
        let right = init.config.width as f32;
        let top = init.config.height as f32;

        let (view_mat, project_mat, _vp) = transforms::create_view_projection_ortho(
            0.0,
            right,
            0.0,
            top,
            -2.0,
            3.0,
            camera_position,
            look_direction,
            up_direction,
        );

        let uniform_buffer = init.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Uniform Buffer"),
            size: 2 * 16 * 4,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let vertex_data = vec![
            -particle_size / 2.0,
            -particle_size / 2.0,
            particle_size / 2.0,
            -particle_size / 2.0,
            -particle_size / 2.0,
            particle_size / 2.0,
            particle_size / 2.0,
            particle_size / 2.0,
        ]
        .to_vec();

        let vertex_buffer = init
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("Vertex Buffer")),
                contents: bytemuck::cast_slice(&vertex_data),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let render_bind_group_layout =
            init.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                    label: None,
                });

        let render_bind_group = init.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &render_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: Some("Render Bind Group"),
        });

        let render_pipeline_layout =
            init.device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("render"),
                    bind_group_layouts: &[&render_bind_group_layout],
                    push_constant_ranges: &[],
                });

        let render_pipeline = init
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[
                        wgpu::VertexBufferLayout {
                            array_stride: 2 * 4,
                            step_mode: wgpu::VertexStepMode::Vertex,
                            attributes: &wgpu::vertex_attr_array![0 => Float32x2],
                        }, 
                        wgpu::VertexBufferLayout {
                            array_stride: (2+2+1+3) * 4,
                            step_mode: wgpu::VertexStepMode::Instance,
                            attributes: &wgpu::vertex_attr_array![1 => Float32x2, 2 => Float32x2, 3 => Float32x3, 4 => Float32],
                        },
                    ],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(init.config.format.into())],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleStrip,
                    strip_index_format: Some(wgpu::IndexFormat::Uint32),
                    ..Default::default()
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
            });

        let work_group_count =
            ((num_particles as f32) / (PARTICLES_PER_GROUP as f32)).ceil() as u32;

        Self {
            init,

            //Compute
            particle_buffer,
            particle_uniform_data,
            particle_uniform_buffer,
            compute_bind_group,
            compute_pipeline,
            work_group_count,
            num_particles,

            // render
            view_mat,
            project_mat,
            uniform_buffer,
            vertex_buffer,
            render_pipeline,
            render_bind_group,

            // parameters
            unif_mp,
            rng,
            start,
            t0: 0.0,
            t1: 0.0,
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.init.size = new_size;
            self.init.config.width = new_size.width;
            self.init.config.height = new_size.height;
            self.init
                .surface
                .configure(&self.init.device, &self.init.config);
        }
    }

    #[allow(unused_variables)]
    fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    fn update(&mut self) {
        // empty
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let t = self.start.elapsed().unwrap().as_millis() as f32 / 1000.0;
        let dt0 = t - self.t0;
        if dt0 >= 1.5 {
            for i in 4..12 {
                self.particle_uniform_data[i] = self.unif_mp.sample(&mut self.rng) * 240.0;
            }
            self.t0 = t;
        }
        let dt1 = t - self.t1;
        self.t1 = t;
        self.particle_uniform_data[2] = dt1;

        let output = self.init.surface.get_current_texture()?;

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder =
            self.init
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

        {
            // compute pass
            let mut compute_pass =
                encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: Some("Compute Pass") });
            compute_pass.set_pipeline(&self.compute_pipeline);
            compute_pass.set_bind_group(0, &self.compute_bind_group, &[]);
            compute_pass.dispatch_workgroups(self.work_group_count, 1, 1);
        }
        {
            // render pass
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            let project_ref: &[f32; 16] = self.project_mat.as_ref();
            let view_ref: &[f32; 16] = self.view_mat.as_ref();
            self.init.queue.write_buffer(
                &self.uniform_buffer,
                0,
                bytemuck::cast_slice(project_ref),
            );
            self.init
                .queue
                .write_buffer(&self.uniform_buffer, 64, bytemuck::cast_slice(view_ref));
            self.init.queue.write_buffer(
                &self.particle_uniform_buffer,
                0,
                bytemuck::cast_slice(&self.particle_uniform_data),
            );

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.render_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.particle_buffer.slice(..));
            render_pass.draw(0..4, 0..self.num_particles);
        }
        self.init.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

fn main() {
    let mut num_particles = "100000";
    let mut size = "2.0";

    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        num_particles = &args[1];
    }
    let np = num_particles.parse::<u32>().unwrap();
    if args.len() > 2 {
        size = &args[2];
    }
    let sz = size.parse::<f32>().unwrap();
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window.set_title(&*format!("ch13_particles"));
    let mut state = pollster::block_on(State::new(&window, np, sz));

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => {
            if !state.input(event) {
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(**new_inner_size);
                    }
                    _ => {}
                }
            }
        }
        Event::RedrawRequested(_) => {
            state.update();
            match state.render() {
                Ok(_) => {}
                Err(wgpu::SurfaceError::Lost) => state.resize(state.init.size),
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                Err(e) => eprintln!("{:?}", e),
            }
        }
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        _ => {}
    });
}
