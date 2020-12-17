mod renderer;

use winit::event_loop::EventLoop;
use winit::event_loop::ControlFlow;
use winit::window::Window;
use winit::event::Event;
use winit::event::WindowEvent;

const APP_NAME : &'static str = "Mport";
const ENGINE_NAME : &'static str = "Mport Engine";
const VERSION : &'static u32 = &1;
const VALIDATION_ENABLED : &'static bool = &true; //This severely hurts performance, only use for debugging!!

fn main(){
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).expect("Failed to create window.");
    let renderer = renderer::Renderer::new(&window);
    let mut first_loop = true;
    event_loop.run(move |event,_,control_flow|{
        *control_flow = ControlFlow::Poll;
        if first_loop{
            first_loop = false;
            renderer.show_create_info();
        }
        match event{
            Event::WindowEvent{
                event : WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    })
}