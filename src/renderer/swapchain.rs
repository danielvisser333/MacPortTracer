use ash::extensions::khr::Swapchain;
use ash::vk::SwapchainKHR;

use ash::Instance;
use ash::Device;
use ash::version::DeviceV1_0;

pub fn get_swapchain_extent(surface_loader : &ash::extensions::khr::Surface , surface : &ash::vk::SurfaceKHR , physical_device : &ash::vk::PhysicalDevice) -> ash::vk::Extent2D{
    let capabilites = unsafe{surface_loader.get_physical_device_surface_capabilities(*physical_device, *surface)}.expect("Failed to acquire surface capabilities.");
    if capabilites.current_extent.width != std::u32::MAX{return capabilites.current_extent}else{return capabilites.max_image_extent};
}
pub fn get_swapchain_surface_format(surface_loader : &ash::extensions::khr::Surface , surface : &ash::vk::SurfaceKHR , physical_device : &ash::vk::PhysicalDevice) -> ash::vk::SurfaceFormatKHR{
    let formats = unsafe{surface_loader.get_physical_device_surface_formats(*physical_device, *surface)}.expect("Failed to acquire supported formats.");
    for &format in formats.iter(){
        if format.format == ash::vk::Format::B8G8R8A8_SRGB && format.color_space == ash::vk::ColorSpaceKHR::SRGB_NONLINEAR{ return format; }
    }
    return formats[1];
}
pub fn get_swapchain_present_mode(surface_loader : &ash::extensions::khr::Surface , surface : &ash::vk::SurfaceKHR , physical_device : &ash::vk::PhysicalDevice) -> ash::vk::PresentModeKHR{
    let present_modes = unsafe{surface_loader.get_physical_device_surface_present_modes(*physical_device, *surface)}.expect("Failed to acquire surface present modes.");
    if present_modes.contains(&ash::vk::PresentModeKHR::MAILBOX){return ash::vk::PresentModeKHR::MAILBOX}else{return ash::vk::PresentModeKHR::FIFO};
}
pub fn get_min_image_count(surface_loader : &ash::extensions::khr::Surface , surface : &ash::vk::SurfaceKHR , physical_device : &ash::vk::PhysicalDevice) -> u32{
    let capabilites = unsafe{surface_loader.get_physical_device_surface_capabilities(*physical_device, *surface)}.expect("Failed to acquire surface capabilities.");
    if capabilites.max_image_count >= capabilites.min_image_count+1{return capabilites.min_image_count+1}else{return capabilites.max_image_count};
}
pub fn create_swapchain(instance : &Instance , device : &Device , surface : &ash::vk::SurfaceKHR , present_mode : &ash::vk::PresentModeKHR , extent : &ash::vk::Extent2D , format : &ash::vk::SurfaceFormatKHR , min_image_count : u32 , graphics_queue_family : u32 , presentation_queue_family : u32 , surface_loader : &ash::extensions::khr::Surface , physical_device : &ash::vk::PhysicalDevice) -> (Swapchain,SwapchainKHR){
    let capabilites = unsafe{surface_loader.get_physical_device_surface_capabilities(*physical_device, *surface)}.expect("Failed to acquire surface capabilities.");
    let swapchain_loader = Swapchain::new(instance , device);
    let swapchain_create_info = ash::vk::SwapchainCreateInfoKHR{
        s_type : ash::vk::StructureType::SWAPCHAIN_CREATE_INFO_KHR,
        p_next : std::ptr::null(),
        flags : ash::vk::SwapchainCreateFlagsKHR::empty(),
        surface : *surface,
        old_swapchain : SwapchainKHR::null(),
        clipped : 1,
        image_array_layers : 1,
        image_usage : ash::vk::ImageUsageFlags::COLOR_ATTACHMENT,
        composite_alpha : ash::vk::CompositeAlphaFlagsKHR::OPAQUE,
        present_mode : *present_mode,
        image_extent : *extent,
        image_color_space : format.color_space,
        image_format : format.format,
        min_image_count : min_image_count,
        image_sharing_mode : if graphics_queue_family == presentation_queue_family {ash::vk::SharingMode::EXCLUSIVE}else{ash::vk::SharingMode::CONCURRENT},
        queue_family_index_count : if graphics_queue_family == presentation_queue_family {0}else{2},
        p_queue_family_indices : if graphics_queue_family == presentation_queue_family{std::ptr::null()}else{[graphics_queue_family,presentation_queue_family].as_ptr()},
        pre_transform : capabilites.current_transform,
    };
    let swapchain = unsafe{swapchain_loader.create_swapchain(&swapchain_create_info, None)}.expect("Failed to create swapchain.");
    return (swapchain_loader,swapchain);
}
pub fn create_swapchain_images(swapchain_loader : &Swapchain , swapchain : &SwapchainKHR) -> Vec<ash::vk::Image>{
    return unsafe{swapchain_loader.get_swapchain_images(*swapchain)}.expect("Failed to acquire images from the swapchain.");
}
pub fn create_swapchain_image_views(images : &Vec<ash::vk::Image> , device : &Device , format : ash::vk::Format) -> Vec<ash::vk::ImageView>{
    let mut image_views = vec!();
    for &image in images.iter(){
        let image_view_create_info = ash::vk::ImageViewCreateInfo{
            s_type : ash::vk::StructureType::IMAGE_VIEW_CREATE_INFO,
            p_next : std::ptr::null(),
            flags : ash::vk::ImageViewCreateFlags::empty(),
            image : image,
            format : format,
            view_type : ash::vk::ImageViewType::TYPE_2D,
            components : ash::vk::ComponentMapping{a:ash::vk::ComponentSwizzle::A,b:ash::vk::ComponentSwizzle::B,g:ash::vk::ComponentSwizzle::G,r:ash::vk::ComponentSwizzle::R},
            subresource_range : ash::vk::ImageSubresourceRange{
                aspect_mask : ash::vk::ImageAspectFlags::COLOR,
                layer_count : 1,
                level_count : 1,
                base_array_layer : 0,
                base_mip_level : 0,
            },
        };
        image_views.push(unsafe{device.create_image_view(&image_view_create_info, None)}.expect("Failed to create image view for the swapchain."));
    }
    return image_views;
}