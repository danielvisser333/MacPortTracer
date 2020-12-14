mod camera;
mod renderer;

use winit::event_loop::EventLoop;
use winit::window::Window;
use winit::event_loop::ControlFlow;
use winit::event::Event;
use winit::event::WindowEvent;

const CONFIG_FILE : &'static str = "config.ini";
const TITLE : &'static str = "MPort";

fn main(){
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).expect("Failed to create window instance.");
    window.set_title(TITLE);
    let renderer = renderer::Renderer::new(&window);
    let mut first_loop = true;
    event_loop.run(move |event,_,control_flow|{
        *control_flow = ControlFlow::Poll;
        if first_loop{
            renderer.vulkan_interface.print_debug_info();
            first_loop = false;
        }
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}