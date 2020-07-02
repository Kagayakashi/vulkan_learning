mod my_graphics;

use vulkano::instance::Instance;
use vulkano::instance::InstanceExtensions;
use vulkano::instance::PhysicalDevice;

use vulkano::device::Device;
use vulkano::device::DeviceExtensions;
use vulkano::device::Features;

use vulkano::buffer::BufferUsage;
use vulkano::buffer::CpuAccessibleBuffer;

use vulkano::command_buffer::{AutoCommandBufferBuilder, AutoCommandBuffer};
use vulkano::command_buffer::CommandBuffer;

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

    let physical = PhysicalDevice::enumerate(&instance)
        .next()
        .expect("no device available");

    let queue_family = physical.queue_families()
        .find(|&q| q.supports_graphics())
        .expect("couldn't find a graphical queue family");
    let (device, mut queues) = {
        Device::new(physical, &Features::none(), &DeviceExtensions::none(),
                    [(queue_family, 0.5)].iter().cloned()).expect("failed to create device")
    };
    let queue = queues.next().unwrap();

    /*
    BUFFER
     */
    struct MyStruct {
        a: u32,
        b: bool,
    }

    let data = MyStruct { a: 5, b: true };
    let buffer = CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), false, data)
        .expect("Failed to create buffer");

    let mut content = buffer.write().unwrap();

    println!("buffer content a{}",content.a);
    println!("buffer content b{}",content.b);

    content.a *= 2;
    content.b = false;

    println!("buffer content a{}",content.a);
    println!("buffer content b{}",content.b);
    // BUFFER

    /*
    COMMAND BUFFER
     */
    let source_content = 0 .. 64;
    let source = CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), false, source_content)
        .expect("Failed to create Source buffer");

    let dest_content = 0 .. 64;
    let dest = CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), false, dest_content)
        .expect("Failed to create Source buffer");

    let mut builder = AutoCommandBufferBuilder::new(device.clone(), queue.family())
        .unwrap();
    builder.copy_buffer(source.clone(), dest.clone())
        .unwrap();

    let command_buffer = builder.build()
        .unwrap();
    // Sumbission and Synchronization
    let finished = command_buffer.execute(queue.clone())
        .unwrap();
    // TODO
    // COMMAND BUFFER



    let events_loop = EventLoop::new();
    let surface = WindowBuilder::new()
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