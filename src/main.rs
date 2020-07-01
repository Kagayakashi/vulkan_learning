mod my_graphics;

use vulkano::instance::Instance;
use vulkano::instance::InstanceExtensions;
use vulkano::instance::PhysicalDevice;
use vulkano_win::VkSurfaceBuild;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder},
};

fn main() {
    println!("Hello, world!");

    my_graphics::my_render::my_vulkan::test();

    let instance = {
        let extensions = vulkano_win::required_extensions();
        Instance::new(None, &extensions, None)
            .expect("failed to create Vulkan instance")
    };

    let _physical = PhysicalDevice::enumerate(&instance)
        .next()
        .expect("no device available");

    let events_loop = EventLoop::new();
    let _surface = WindowBuilder::new()
        .with_title("My First Window")
        .with_resizable(false )
        .build_vk_surface(&events_loop, instance.clone())
        .unwrap();

    events_loop.run(|event, _, control_flow| {
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            },
            _ => ()
        }
    });
}