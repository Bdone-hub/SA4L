use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window as WinitWindow, WindowBuilder},
};
use crate::context::Context;

pub struct Window {
    event_loop: EventLoop<()>,
    window: WinitWindow,
    context: Context,
}

impl Window {
    pub fn new() -> Self {
        env_logger::init();

        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        let context = pollster::block_on(Context::new(&window));

        Self {
            event_loop,
            window,
            context,
        }
    }

    pub fn run<F>(self, mut render: F)
    where F: FnMut(&Context) -> Result<(), wgpu::SurfaceError> + 'static
    {
        let window = self.window;
        let mut context = self.context;
        self.event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == window.id() => {
                    if true {
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
                                context.resize(*physical_size);
                            }
                            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                                // new_inner_size is &mut so w have to dereference it twice
                                context.resize(**new_inner_size);
                            }
                            _ => {}
                        }
                    }
                }
                Event::RedrawRequested(window_id) if window_id == window.id() => {
                    match render(&context) {
                        Ok(_) => {}
                        // Reconfigure the surface if it's lost or outdated
                        Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => context.resize(context.size),
                        // The system is out of memory, we should probably quit
                        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        // We're ignoring timeouts
                        Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                    }
                }
                Event::MainEventsCleared => {
                    // RedrawRequested will only trigger once, unless we manually
                    // request it.
                    window.request_redraw();
                }
                _ => {}
            }
        });
    }

    pub fn get_context(&self) -> &Context {
        return &self.context;
    }
}