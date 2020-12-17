use ash::extensions::khr::Surface;
use ash::vk::SurfaceKHR;

use ash::Entry;
use ash::Instance;

use winit::window::Window;

pub fn create_surface(entry : &Entry, instance : &Instance, window : &Window) -> (Surface,SurfaceKHR){
    let surface_loader = Surface::new(entry,instance);
    let surface = super::platform::create_surface(entry , instance , window);
    return (surface_loader,surface);
}