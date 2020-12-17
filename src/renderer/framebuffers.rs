use ash::Device;
use ash::version::DeviceV1_0;

pub fn create_framebuffers(image_views : &Vec<ash::vk::ImageView> , device : &Device , extent : &ash::vk::Extent2D , render_pass : &ash::vk::RenderPass) -> Vec<ash::vk::Framebuffer>{
    let mut framebuffers = vec!();
    for &image_view in image_views.iter(){
        let attachments = [image_view];
        let framebuffer_create_info = ash::vk::FramebufferCreateInfo{
            s_type : ash::vk::StructureType::FRAMEBUFFER_CREATE_INFO,
            p_next : std::ptr::null(),
            flags : ash::vk::FramebufferCreateFlags::empty(),
            width : extent.width,
            height : extent.height,
            attachment_count : attachments.len() as u32,
            p_attachments : attachments.as_ptr(),
            render_pass : *render_pass,
            layers : 1,
        };
        framebuffers.push(unsafe{device.create_framebuffer(&framebuffer_create_info, None)}.expect("Failed to create framebuffer."));
    }
    return framebuffers;
}