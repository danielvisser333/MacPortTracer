mod instance;
mod surface;
mod device;
mod swapchain;
mod render_pass;
mod framebuffers;

use winit::window::Window;

use ash::version::DeviceV1_0;
use ash::version::InstanceV1_0;

pub struct Renderer{
    _entry : ash::Entry,
    instance : ash::Instance,
    surface_loader : ash::extensions::khr::Surface,
    surface : ash::vk::SurfaceKHR,
    physical_device : ash::vk::PhysicalDevice,
    graphics_queue_family : u32,
    presentation_queue_family : u32,
    transfer_queue_family : u32,
    compute_queue_family : u32,
    device : ash::Device,
    present_mode : ash::vk::PresentModeKHR,
    swapchain_format : ash::vk::SurfaceFormatKHR,
    swapchain_extent : ash::vk::Extent2D,
    swapchain_image_count : u32,
    swapchain_loader : ash::extensions::khr::Swapchain,
    swapchain : ash::vk::SwapchainKHR,
    _swapchain_images : Vec<ash::vk::Image>,
    swapchain_image_views : Vec<ash::vk::ImageView>,
    render_pass : ash::vk::RenderPass,
    framebuffers : Vec<ash::vk::Framebuffer>,
}
impl Renderer{
    pub fn new(window : &Window)->Self{
        let entry = instance::create_entry();
        let instance = instance::create_instance(&entry,window);
        let (surface_loader,surface) = surface::create_surface(&entry, &instance, window);
        let physical_device = device::choose_physical_device(&instance, &surface_loader, &surface);
        let graphics_queue_family = device::get_graphics_queue_family(&instance, &physical_device);
        let presentation_queue_family = device::get_presentation_queue_family(&instance, &physical_device, &surface_loader, &surface);
        let transfer_queue_family = device::get_transfer_queue_family(&instance, &physical_device);
        let compute_queue_family = device::get_compute_queue_family(&instance, &physical_device);
        let device = device::create_device(&instance, &physical_device, graphics_queue_family, transfer_queue_family, compute_queue_family, presentation_queue_family);
        let present_mode = swapchain::get_swapchain_present_mode(&surface_loader, &surface, &physical_device);
        let format = swapchain::get_swapchain_surface_format(&surface_loader, &surface, &physical_device);
        let extent = swapchain::get_swapchain_extent(&surface_loader, &surface, &physical_device);
        let min_image_count = swapchain::get_min_image_count(&surface_loader, &surface, &physical_device);
        let (swapchain_loader,swapchain) = swapchain::create_swapchain(&instance, &device, &surface, &present_mode, &extent, &format, min_image_count, graphics_queue_family, presentation_queue_family, &surface_loader, &physical_device);
        let swapchain_images = swapchain::create_swapchain_images(&swapchain_loader, &swapchain);
        let swapchain_image_views = swapchain::create_swapchain_image_views(&swapchain_images, &device, format.format);
        let render_pass = render_pass::create_render_pass(&device, format.format);
        let framebuffers = framebuffers::create_framebuffers(&swapchain_image_views, &device, &extent, &render_pass);
        return Self{
            _entry : entry,
            instance,
            surface_loader,
            surface,
            physical_device,
            graphics_queue_family,
            compute_queue_family,
            transfer_queue_family,
            presentation_queue_family,
            device,
            present_mode,
            swapchain_format : format,
            swapchain_extent : extent,
            swapchain_image_count : min_image_count,
            swapchain_loader,
            swapchain,
            _swapchain_images : swapchain_images,
            swapchain_image_views,
            render_pass,
            framebuffers,
        }
    }
    pub fn show_create_info(&self){
        let device_properties = unsafe{self.instance.get_physical_device_properties(self.physical_device)};
        let raw_string = unsafe {
            let pointer = device_properties.device_name.as_ptr();
            std::ffi::CStr::from_ptr(pointer)
        };
        let device_type = match device_properties.device_type {
            ash::vk::PhysicalDeviceType::CPU => "Cpu",
            ash::vk::PhysicalDeviceType::INTEGRATED_GPU => "Integrated GPU",
            ash::vk::PhysicalDeviceType::DISCRETE_GPU => "Discrete GPU",
            ash::vk::PhysicalDeviceType::VIRTUAL_GPU => "Virtual GPU",
            ash::vk::PhysicalDeviceType::OTHER => "Unknown",
            _ => panic!(),
        };
        let device_name = raw_string.to_str().expect("Failed to convert vulkan raw string.").to_owned();
        println!("Name : {}, version : {}.",super::APP_NAME,super::VERSION);
        println!("Using engine : {}.",super::ENGINE_NAME);
        println!("");
        println!("Vulkan info : ");
        println!("Using device : {} of type {}.",device_name,device_type);
        println!("");
        println!("Graphics queue family : {}.",self.graphics_queue_family);
        println!("Presentation queue family : {}.",self.presentation_queue_family);
        println!("Transfer queue family : {}.",self.transfer_queue_family);
        println!("Compute queue family : {}.",self.compute_queue_family);
        println!("");
        println!("Using Swapchain with {} images.",self.swapchain_image_count);
        println!("Using Swapchain present mode : {}." , if self.present_mode == ash::vk::PresentModeKHR::MAILBOX{"Mailbox"}else{"FIFO"});
        println!("Using Swapchain Extent : x : {} , y : {}.",self.swapchain_extent.width,self.swapchain_extent.height);
        println!("Using Swapchain Format : {:?}, and Color space : {:?}.",self.swapchain_format.format,self.swapchain_format.color_space);
        println!("Using Render pass with 1 Subpass.");
        println!("");
    }
    pub fn recreate_swapchain(&mut self){
        unsafe{self.swapchain_loader.destroy_swapchain(self.swapchain, None)};
        let swapchain_tupple = swapchain::create_swapchain(&self.instance, &self.device, &self.surface, &self.present_mode, &self.swapchain_extent, &self.swapchain_format, self.swapchain_image_count, self.graphics_queue_family, self.presentation_queue_family, &self.surface_loader, &self.physical_device);
        self.swapchain = swapchain_tupple.1; self.swapchain_loader = swapchain_tupple.0;
    }
}
impl Drop for Renderer{
    fn drop(&mut self){
        unsafe{self.device.device_wait_idle()}.expect("Oh no! The renderer crashed.");
        for &framebuffer in self.framebuffers.iter(){
            unsafe{self.device.destroy_framebuffer(framebuffer, None)};
        }
        unsafe{self.device.destroy_render_pass(self.render_pass, None)};
        for &image_view in self.swapchain_image_views.iter(){
            unsafe{self.device.destroy_image_view(image_view, None)};
        }
        unsafe{self.swapchain_loader.destroy_swapchain(self.swapchain, None)};
        unsafe{self.device.destroy_device(None)};
        unsafe{self.surface_loader.destroy_surface(self.surface, None)};
        unsafe{self.instance.destroy_instance(None)};
    }
}