mod vulkan;
pub struct Renderer{
    camera : super::camera::Camera,
    width : u32,
    height : u32,
    vulkan_interface : vulkan::VulkanInterface,
}
impl Renderer{
    pub fn new(window : &winit::window::Window) -> Self{
        let size = window.inner_size();
        let vulkan_interface = vulkan::VulkanInterface::new(true,window);
        let camera = super::camera::Camera::default(cgmath::Vector2::new(size.width as f32,size.height as f32));
        return Self{
            camera : camera,
            width : size.width,
            height : size.height,
            vulkan_interface : vulkan_interface,
        }
    }
}