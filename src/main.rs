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
use vulkano_win::CreationError::WindowCreationError;

struct HelloTriangleApplication {
    event_loop: EventLoop<()>,
    window: Window,
}

impl HelloTriangleApplication {
    pub fn run(&mut self){
        self.init_window();
        self.init_vulkan();
        self.init_loop();

        println!("Application started successful!");
    }

    fn init_window(&mut self) {
        println!("Window initialization...");
        /*
        let event_loop = EventLoop::new();
        let _window = WindowBuilder::new()
            .build(&event_loop)
            .unwrap();
        self.event_loop = event_loop;
        */
        self.event_loop = EventLoop::new();
        self.window = Window::new(&self.event_loop)
            .unwrap();
    }

    fn init_vulkan(&mut self) {
        println!("Vulkan initialization...");
    }

    fn init_loop(&mut self){
        println!("Loop initialization...");

        self.event_loop.run(move |event, _, control_flow| {
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
                    self.window.request_redraw();
                },
                Event::RedrawRequested(_) => {
                    // Redraw the application.
                },
                _ => ()
            }
        });
    }
}

fn main() {

    // Include my_graphics/my_render/my_vulkan/mod.rs
    my_graphics::my_render::my_vulkan::test();
    //--------------------------------------------------------------------------------------------//
    //let app = HelloTriangleApplication::initialize();
    let mut app = HelloTriangleApplication;
    app.run();

    /*
    let event_loop = EventLoop::new();
    let _window = WindowBuilder::new()
        .build(&event_loop)
        .unwrap();
    event_loop.run(move |event, _, control_flow| {
        // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
        // dispatched any events. This is ideal for games and similar applications.
        *control_flow = ControlFlow::Poll;

        // ControlFlow::Wait pauses the event loop if no events are available to process.
        // This is ideal for non-game applications that only update in response to user
        // input, and uses significantly less power/CPU time than ControlFlow::Poll.
        *control_flow = ControlFlow::Wait;

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
                //
                // You only need to call this if you've determined that you need to redraw, in
                // applications which do not always need to. Applications that redraw continuously
                // can just render here instead.
                _window.request_redraw();
            },
            Event::RedrawRequested(_) => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in MainEventsCleared, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.
            },
            _ => ()
        }
    });
     */
}