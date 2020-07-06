mod my_graphics;

use vulkano::instance::Instance;
use vulkano::instance::InstanceExtensions;

use vulkano_win::VkSurfaceBuild;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{
        Window,
    },
};

struct HelloTriangleApplication { /* No fields */ }

impl HelloTriangleApplication {
    pub fn run(&mut self){
        self.init_window();
        self.init_vulkan();
        self.init_loop();

        println!("Application started successful!");
    }

    fn init_window(&mut self) {
        println!("Window initialization...");
    }

    fn init_vulkan(&mut self) {
        println!("Vulkan initialization...");
    }

    fn init_loop(&mut self) {
        println!("Loop initialization...");
    }
}

fn main() {

    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop)
        .unwrap();
    event_loop.run(move |event, _, control_flow| {
        // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
        // dispatched any events. This is ideal for games and similar applications.
        *control_flow = ControlFlow::Poll;

        // ControlFlow::Wait pauses the event loop if no events are available to process.
        // This is ideal for non-game applications that only update in response to user
        // input, and uses significantly less power/CPU time than ControlFlow::Poll.
        //*control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                *control_flow = ControlFlow::Exit
            },
            Event::MainEventsCleared => {
                // Application update code.
                // Queue a RedrawRequested event.
                window.request_redraw();
            },
            Event::RedrawRequested(_) => {
                // Redraw the application.
            },
            _ => (),
        }
    });
}