use ash::Device;
use ash::version::DeviceV1_0;

pub fn create_render_pass(device : &Device , format : ash::vk::Format) -> ash::vk::RenderPass{
    let color_attachment = ash::vk::AttachmentDescription{
        flags : ash::vk::AttachmentDescriptionFlags::empty(),
        format : format,
        initial_layout : ash::vk::ImageLayout::UNDEFINED,
        final_layout : ash::vk::ImageLayout::PRESENT_SRC_KHR,
        load_op : ash::vk::AttachmentLoadOp::CLEAR,
        store_op : ash::vk::AttachmentStoreOp::STORE,
        stencil_load_op : ash::vk::AttachmentLoadOp::DONT_CARE,
        stencil_store_op : ash::vk::AttachmentStoreOp::DONT_CARE,
        samples : ash::vk::SampleCountFlags::TYPE_1,
    };
    let attachments = [color_attachment];
    let color_attachment_references = [ash::vk::AttachmentReference{
        attachment : 0,
        layout : ash::vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
    }];
    let subpass = ash::vk::SubpassDescription{
        pipeline_bind_point : ash::vk::PipelineBindPoint::GRAPHICS,
        color_attachment_count : color_attachment_references.len() as u32,
        p_color_attachments : color_attachment_references.as_ptr(),
        p_depth_stencil_attachment : std::ptr::null(),
        input_attachment_count : 0,
        p_input_attachments : std::ptr::null(),
        preserve_attachment_count : 0,
        p_preserve_attachments : std::ptr::null(),
        p_resolve_attachments : std::ptr::null(),
        flags : ash::vk::SubpassDescriptionFlags::empty(),
    };
    let subpass_dependency = ash::vk::SubpassDependency{
        dependency_flags : ash::vk::DependencyFlags::empty(),
        src_subpass : ash::vk::SUBPASS_EXTERNAL,
        dst_subpass : 0,
        src_stage_mask : ash::vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
        dst_stage_mask : ash::vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
        src_access_mask : ash::vk::AccessFlags::empty(),
        dst_access_mask : ash::vk:: AccessFlags::COLOR_ATTACHMENT_WRITE,
    };
    let dependencies = [subpass_dependency];
    let render_pass_create_info = ash::vk::RenderPassCreateInfo{
        s_type : ash::vk::StructureType::RENDER_PASS_CREATE_INFO,
        p_next : std::ptr::null(),
        flags : ash::vk::RenderPassCreateFlags::empty(),
        subpass_count : 1,
        p_subpasses : &subpass,
        attachment_count : attachments.len() as u32,
        p_attachments : attachments.as_ptr(),
        dependency_count : dependencies.len() as u32,
        p_dependencies : dependencies.as_ptr(),
    };
    return unsafe{device.create_render_pass(&render_pass_create_info, None)}.expect("Failed to create render pass.");
}