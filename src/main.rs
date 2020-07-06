mod my_graphics;

use vulkano::instance::Instance;
use vulkano::instance::InstanceExtensions;

use vulkano_win::VkSurfaceBuild;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{
        WindowBuilder,
    },
};

use vulkano::instance::debug::DebugCallback;

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


    let instance = {
        let extensions = vulkano_win::required_extensions();
        Instance::new(None, &extensions, None)
            .expect("failed to create Vulkan instance")
    };

    let _callback = DebugCallback::errors_and_warnings(&instance, |msg| {
        println!("Debug callback: {:?}", msg.description);
    }).ok();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("My Window")
        .build_vk_surface(&event_loop, instance.clone())
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
            _ => (),
        }
    });
}