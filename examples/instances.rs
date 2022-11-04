use wgpu_bootstrap::{
    window::Window,
    frame::Frame,
    cgmath::prelude::*,
    application::Application,
    texture::create_simple_texture_bind_group,
    context::Context,
    camera::Camera,
    default::{ SimpleVertex, Instance, InstanceRaw },
    wgpu,
};

const VERTICES: &[SimpleVertex] = &[
    SimpleVertex { position: [-0.0868241, 0.49240386, 0.0], tex_coords: [0.4131759, 0.00759614], },
    SimpleVertex { position: [-0.49513406, 0.06958647, 0.0], tex_coords: [0.0048659444, 0.43041354], },
    SimpleVertex { position: [-0.21918549, -0.44939706, 0.0], tex_coords: [0.28081453, 0.949397], },
    SimpleVertex { position: [0.35966998, -0.3473291, 0.0], tex_coords: [0.85967, 0.84732914], },
    SimpleVertex { position: [0.44147372, 0.2347359, 0.0], tex_coords: [0.9414737, 0.2652641], },
];

const INDICES: &[u16] = &[
    0, 1, 4,
    1, 2, 4,
    2, 3, 4,
];

struct MyApp {
    diffuse_bind_group: wgpu::BindGroup,
    camera_bind_group: wgpu::BindGroup,
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    instances: Vec<Instance>,
    instance_buffer: wgpu::Buffer,
}

impl MyApp {
    fn new(context: &Context) -> Self {
        let texture = context.create_texture("happy-tree.png", include_bytes!("happy-tree.png"));
    
        let (texture_bind_group_layout, diffuse_bind_group) = create_simple_texture_bind_group(context, &texture);
    
        let camera = Camera {
            eye: (0.0, 10.0, 15.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: cgmath::Vector3::unit_y(),
            aspect: context.get_aspect_ratio(),
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        };

        let (_camera_buffer, camera_bind_group_layout, camera_bind_group) = camera.create_camera_bind_group(context);
    
        let pipeline = context.create_render_pipeline("Render Pipeline", include_str!("shader_instances.wgsl"), &[SimpleVertex::desc(), InstanceRaw::desc()], &[
            &texture_bind_group_layout,
            &camera_bind_group_layout,
        ]);
    
        let vertex_buffer = context.create_buffer(VERTICES, wgpu::BufferUsages::VERTEX);
        let index_buffer = context.create_buffer(INDICES, wgpu::BufferUsages::INDEX);

        let NUM_INSTANCES_PER_ROW = 10;
        let INSTANCE_DISPLACEMENT = cgmath::Vector3::new(NUM_INSTANCES_PER_ROW as f32 * 0.5, 0.0, NUM_INSTANCES_PER_ROW as f32 * 0.5);

        let instances = (0..NUM_INSTANCES_PER_ROW*NUM_INSTANCES_PER_ROW).map(|index| {
            let x = index % NUM_INSTANCES_PER_ROW;
            let z = index / NUM_INSTANCES_PER_ROW;
            let position = cgmath::Vector3 { x: x as f32, y: 0.0, z: z as f32 } - INSTANCE_DISPLACEMENT;
            let rotation = if position.is_zero() {
                // this is needed so an object at (0, 0, 0) won't get scaled to zero
                // as Quaternions can effect scale if they're not created correctly
                cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0))
            } else {
                cgmath::Quaternion::from_axis_angle(position.normalize(), cgmath::Deg(45.0))
            };

            Instance {
                position, rotation,
            }
        }).collect::<Vec<_>>();

        let instance_data = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
        let instance_buffer = context.create_buffer(instance_data.as_slice(), wgpu::BufferUsages::VERTEX);
        
        Self {
            diffuse_bind_group,
            camera_bind_group,
            pipeline,
            vertex_buffer,
            index_buffer,
            instances,
            instance_buffer
        }
    }
}

impl Application for MyApp {
    fn render(&self, context: &Context) -> Result<(), wgpu::SurfaceError> {
        let mut frame = Frame::new(context)?;

        {
            let mut render_pass = frame.begin_render_pass();

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
            render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..(INDICES.len() as u32), 0, 0..self.instances.len() as _);
        }

        frame.present();

        Ok(())
    }
}

fn main() {
    let window = Window::new();

    let context = window.get_context();

    let my_app = MyApp::new(context);

    window.run(my_app);
}
