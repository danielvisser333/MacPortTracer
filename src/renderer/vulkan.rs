use ash::Entry;
use ash::version::EntryV1_0;
use ash::Instance;
use ash::version::InstanceV1_0;
use ash::Device;
use ash::version::DeviceV1_0;
use ash::vk;
use ash::extensions::khr::Surface;
use ash::extensions::khr::Swapchain;

use std::ffi::CString;

pub struct VulkanInterface{
    entry : Entry,
    instance : Instance,
    surface_loader : Surface,
    surface : vk::SurfaceKHR,
    physical_device : vk::PhysicalDevice,
    graphics_queue : u32,
    presentation_queue : u32,
    transfer_queue : u32,
    compute_queue : u32,
    device : Device,
    swapchain_loader : Swapchain,
    swapchain : vk::SwapchainKHR,
    swapchain_images : Vec<vk::Image>,
    swapchain_image_views : Vec<vk::ImageView>,
    render_pass : vk::RenderPass,
    framebuffers : Vec<vk::Framebuffer>,
}
impl VulkanInterface{
    pub fn new(validation_enabled : bool , window : &winit::window::Window)->Self{
        let entry = Entry::new().expect("Vulkan is not installed, please update your drivers.");
        let application_info = vk::ApplicationInfo{
            s_type : vk::StructureType::APPLICATION_INFO,
            p_next : std::ptr::null(),
            api_version : vk::make_version(1, 2, 0),
            engine_version : vk::make_version(0, 0, 1),
            application_version : vk::make_version(0, 0, 1),
            p_engine_name : CString::new(super::super::TITLE).unwrap().as_ptr(),
            p_application_name : CString::new(super::super::TITLE).unwrap().as_ptr(),
        };
        let validation_layer_name = if validation_enabled{vec!(CString::new("VK_LAYER_KHRONOS_validation").unwrap())} else {vec!()};
        let validation_layer_name_raw : Vec<*const i8> = validation_layer_name.iter().map(|name| name.as_ptr()).collect();
        let mut extensions : Vec<*const i8> = Vec::new();
            extensions.push(ash::extensions::khr::Surface::name().as_ptr());
            #[cfg(all(windows))]
            extensions.push(ash::extensions::khr::Win32Surface::name().as_ptr());
            #[cfg(target_os = "macos")]
            extensions.push(ash::extensions::mvk::MacOSSurface::name().as_ptr());
            #[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
            extensions.push(ash::extensions::khr::XlibSurface::name().as_ptr());
        
        if validation_enabled {extensions.push(ash::extensions::ext::DebugUtils::name().as_ptr())}
        let instance_create_info = vk::InstanceCreateInfo{
            s_type : vk::StructureType::INSTANCE_CREATE_INFO,
            p_next : std::ptr::null(),
            flags : vk::InstanceCreateFlags::empty(),
            enabled_layer_count : validation_layer_name.len() as u32,
            enabled_extension_count : extensions.len() as u32,
            pp_enabled_extension_names : extensions.as_ptr(),
            pp_enabled_layer_names :validation_layer_name_raw.as_ptr(),
            p_application_info : &application_info
        };
        let instance = unsafe{entry.create_instance(&instance_create_info,None)}.expect("Failed to create Vulkan Instance, are your drivers out of date or are you using a debug version?");
        let surface_loader = Surface::new(&entry, &instance);
        let surface = unsafe{Self::create_surface(&entry, &instance, &window)};
        let physical_device = Self::choose_phyical_device(&instance, &surface_loader, &surface);
        let mut graphics_queue = None;
        let mut presentation_queue = None;
        let mut transfer_queue = None;
        let mut compute_queue = None;
        let queue_families = unsafe{instance.get_physical_device_queue_family_properties(physical_device)};
        for (index,queue_family) in queue_families.iter().enumerate(){
            if graphics_queue.is_none() && queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS){graphics_queue=Some(index as u32)}
            if transfer_queue.is_none() && queue_family.queue_flags.contains(vk::QueueFlags::TRANSFER){transfer_queue=Some(index as u32)}
            if compute_queue.is_none() && queue_family.queue_flags.contains(vk::QueueFlags::COMPUTE){compute_queue=Some(index as u32)};
            if presentation_queue.is_none() && unsafe{surface_loader.get_physical_device_surface_support(physical_device, index as u32, surface)}.expect("Failed to query presentation support."){presentation_queue=Some(index as u32)}
            if compute_queue.is_some() && queue_family.queue_flags.contains(vk::QueueFlags::COMPUTE) && queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS){compute_queue=Some(index as u32)}
            if graphics_queue.is_some() && queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS) && !queue_family.queue_flags.contains(vk::QueueFlags::TRANSFER){graphics_queue = Some(index as u32)}
            if transfer_queue.is_some() && queue_family.queue_flags.contains(vk::QueueFlags::TRANSFER) && !queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS){transfer_queue=Some(index as u32)}
        }
        let graphics_queue = graphics_queue.unwrap();
        let transfer_queue = transfer_queue.unwrap();
        let presentation_queue = presentation_queue.unwrap();
        let compute_queue = compute_queue.unwrap();
        let priorities = [1.0f32];
        let mut queues = Vec::new();
        queues.push(graphics_queue);
        if !queues.contains(&transfer_queue){queues.push(transfer_queue)}
        if !queues.contains(&presentation_queue){queues.push(presentation_queue)}
        if !queues.contains(&compute_queue){queues.push(compute_queue)}
        let mut queue_infos = Vec::new();
        for queue in queues{
            queue_infos.push(vk::DeviceQueueCreateInfo{
                s_type : vk::StructureType::DEVICE_QUEUE_CREATE_INFO,
                p_next : std::ptr::null(),
                flags : vk::DeviceQueueCreateFlags::empty(),
                queue_count : 1,
                queue_family_index : queue,
                p_queue_priorities : priorities.as_ptr(),
            })
        }
        let extensions = [ash::extensions::khr::Swapchain::name().as_ptr()];
        let features = vk::PhysicalDeviceFeatures{
            ..Default::default()
        };
        let device_create_info = vk::DeviceCreateInfo{
            s_type : vk::StructureType::DEVICE_CREATE_INFO,
            p_next : std::ptr::null(),
            flags : vk::DeviceCreateFlags::empty(),
            enabled_extension_count : extensions.len() as u32,
            pp_enabled_extension_names : extensions.as_ptr(),
            enabled_layer_count : 0,
            pp_enabled_layer_names : std::ptr::null(),
            queue_create_info_count : queue_infos.len() as u32,
            p_queue_create_infos : queue_infos.as_ptr(),
            p_enabled_features : &features,
        };
        let device = unsafe{instance.create_device(physical_device, &device_create_info, None)}.expect("Failed to create device.");
        let swapchain_loader = Swapchain::new(&instance,&device);
        let present_mode = if unsafe{surface_loader.get_physical_device_surface_present_modes(physical_device, surface)}.expect("Failed to acquire surface present modes.").contains(&vk::PresentModeKHR::MAILBOX){vk::PresentModeKHR::MAILBOX}else{vk::PresentModeKHR::FIFO};
        let capabilites = unsafe{surface_loader.get_physical_device_surface_capabilities(physical_device, surface)}.expect("Failed to acquire surface capabilities.");
        let min_image_count = if capabilites.max_image_count >= capabilites.min_image_count+1{capabilites.min_image_count+1}else{capabilites.max_image_count};
        let format = Self::choose_format(unsafe{surface_loader.get_physical_device_surface_formats(physical_device, surface)}.expect("Failed to acquire supported formats."));
        let swapchain_create_info = vk::SwapchainCreateInfoKHR{
            s_type : vk::StructureType::SWAPCHAIN_CREATE_INFO_KHR,
            p_next : std::ptr::null(),
            flags : vk::SwapchainCreateFlagsKHR::empty(),
            clipped : 1,
            image_array_layers : 1,
            image_color_space : format.color_space,
            image_format : format.format,
            image_sharing_mode : if graphics_queue == presentation_queue{vk::SharingMode::EXCLUSIVE} else {vk::SharingMode::CONCURRENT},
            image_usage : vk::ImageUsageFlags::COLOR_ATTACHMENT,
            min_image_count : min_image_count,
            queue_family_index_count : if presentation_queue == graphics_queue{0} else {2},
            p_queue_family_indices : if presentation_queue == graphics_queue{std::ptr::null()}else{[graphics_queue,presentation_queue].as_ptr()},
            old_swapchain : vk::SwapchainKHR::null(),
            composite_alpha : vk::CompositeAlphaFlagsKHR::OPAQUE,
            present_mode : present_mode,
            surface : surface,
            image_extent : if capabilites.current_extent.width != std::u32::MAX{capabilites.current_extent}else{capabilites.max_image_extent},
            pre_transform : capabilites.current_transform,
        };
        let swapchain = unsafe{swapchain_loader.create_swapchain(&swapchain_create_info, None)}.expect("Failed to create swapchain.");
        let mut swapchain_image_views = Vec::new();
        let swapchain_images = unsafe{swapchain_loader.get_swapchain_images(swapchain)}.expect("Failed to acquire swapchain images.");
        for &image in swapchain_images.iter(){
            let image_view_create_info = vk::ImageViewCreateInfo{
                s_type : vk::StructureType::IMAGE_VIEW_CREATE_INFO,
                p_next : std::ptr::null(),
                flags : vk::ImageViewCreateFlags::empty(),
                image : image,
                format : format.format,
                view_type : vk::ImageViewType::TYPE_2D,
                components : vk::ComponentMapping{a:vk::ComponentSwizzle::A,b:vk::ComponentSwizzle::B,g:vk::ComponentSwizzle::G,r:vk::ComponentSwizzle::R},
                subresource_range : vk::ImageSubresourceRange{
                    aspect_mask : vk::ImageAspectFlags::COLOR,
                    layer_count : 1,
                    level_count : 1,
                    base_array_layer : 0,
                    base_mip_level : 0,
                },
            };
            let image_view = unsafe{device.create_image_view(&image_view_create_info, None)}.expect("Failed to create swapchain image view.");
            swapchain_image_views.push(image_view);
        }
        let color_attachment = vk::AttachmentDescription{
            flags : vk::AttachmentDescriptionFlags::empty(),
            format : format.format,
            initial_layout : vk::ImageLayout::UNDEFINED,
            final_layout : vk::ImageLayout::PRESENT_SRC_KHR,
            load_op : vk::AttachmentLoadOp::CLEAR,
            store_op : vk::AttachmentStoreOp::STORE,
            stencil_load_op : vk::AttachmentLoadOp::DONT_CARE,
            stencil_store_op : vk::AttachmentStoreOp::DONT_CARE,
            samples : vk::SampleCountFlags::TYPE_1,
        };
        let attachments = [color_attachment];
        let color_attachment_references = [vk::AttachmentReference{
            attachment : 0,
            layout : vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
        }];
        let subpass = vk::SubpassDescription{
            pipeline_bind_point : vk::PipelineBindPoint::GRAPHICS,
            color_attachment_count : color_attachment_references.len() as u32,
            p_color_attachments : color_attachment_references.as_ptr(),
            p_depth_stencil_attachment : std::ptr::null(),
            input_attachment_count : 0,
            p_input_attachments : std::ptr::null(),
            preserve_attachment_count : 0,
            p_preserve_attachments : std::ptr::null(),
            p_resolve_attachments : std::ptr::null(),
            flags : vk::SubpassDescriptionFlags::empty(),
        };
        let subpass_dependency = vk::SubpassDependency{
            dependency_flags : vk::DependencyFlags::empty(),
            src_subpass : vk::SUBPASS_EXTERNAL,
            dst_subpass : 0,
            src_stage_mask : vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            dst_stage_mask : vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            src_access_mask : vk::AccessFlags::empty(),
            dst_access_mask : vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
        };
        let dependencies = [subpass_dependency];
        let render_pass_create_info = vk::RenderPassCreateInfo{
            s_type : vk::StructureType::RENDER_PASS_CREATE_INFO,
            p_next : std::ptr::null(),
            flags : vk::RenderPassCreateFlags::empty(),
            subpass_count : 1,
            p_subpasses : &subpass,
            attachment_count : attachments.len() as u32,
            p_attachments : attachments.as_ptr(),
            dependency_count : dependencies.len() as u32,
            p_dependencies : dependencies.as_ptr(),
        };
        let render_pass = unsafe{device.create_render_pass(&render_pass_create_info, None)}.expect("Failed to create render pass.");
        let mut framebuffers = Vec::new();
        for &image_view in swapchain_image_views.iter(){
            let attachments = [image_view];
            let framebuffer_create_info = vk::FramebufferCreateInfo{
                s_type : vk::StructureType::FRAMEBUFFER_CREATE_INFO,
                p_next : std::ptr::null(),
                flags : vk::FramebufferCreateFlags::empty(),
                width : if capabilites.current_extent.width != std::u32::MAX{capabilites.current_extent}else{capabilites.max_image_extent}.width,
                height : if capabilites.current_extent.width != std::u32::MAX{capabilites.current_extent}else{capabilites.max_image_extent}.height,
                attachment_count : attachments.len() as u32,
                p_attachments : attachments.as_ptr(),
                render_pass : render_pass,
                layers : 1,
            };
            let framebuffer = unsafe{device.create_framebuffer(&framebuffer_create_info, None)}.expect("Failed to create framebuffer.");
            framebuffers.push(framebuffer);
        }
        return Self{
            entry,instance,surface_loader,surface,physical_device,graphics_queue,transfer_queue,presentation_queue,compute_queue,device,
            swapchain_loader,swapchain,swapchain_images,swapchain_image_views, render_pass, framebuffers
        }
    }
    #[cfg(all(windows))]
    unsafe fn create_surface(entry : &Entry , instance : &Instance , window : &winit::window::Window) -> vk::SurfaceKHR{
        use winapi::um::libloaderapi::GetModuleHandleW;
        use winit::platform::windows::WindowExtWindows;
        let hinstance = GetModuleHandleW(std::ptr::null()) as *const std::ffi::c_void;
        let hwnd = window.hwnd();
        let create_info = ash::vk::Win32SurfaceCreateInfoKHR{
            s_type: ash::vk::StructureType::WIN32_SURFACE_CREATE_INFO_KHR,
            p_next: std::ptr::null(),
            flags: Default::default(),
            hinstance,
            hwnd: hwnd,
        };
        ash::extensions::khr::Win32Surface::new(entry, instance).create_win32_surface(&create_info, None).expect("Failed to create Win32 surface.")
    }
    #[cfg(target_os = "macos")]
    unsafe fn create_surface<E: EntryV1_0, I: InstanceV1_0>(entry: &E,instance: &I,window: &winit::window::Window,) -> vk::SurfaceKHR {
        use std::mem;
        use std::os::raw::c_void;
        use std::ptr;
        use winit::platform::macos::WindowExtMacOS;

        let wnd: cocoa_id = mem::transmute(window.ns_window());

        let layer = CoreAnimationLayer::new();

        layer.set_edge_antialiasing_mask(0);
        layer.set_presents_with_transaction(false);
        layer.remove_all_animations();

        let view = wnd.contentView();

        layer.set_contents_scale(view.backingScaleFactor());
        view.setLayer(mem::transmute(layer.as_ref()));
        view.setWantsLayer(YES);

        let create_info = vk::MacOSSurfaceCreateInfoMVK {
            s_type: vk::StructureType::MACOS_SURFACE_CREATE_INFO_M,
            p_next: ptr::null(),
            flags: Default::default(),
            p_view: window.ns_view() as *const c_void,
        };

        let macos_surface_loader = MacOSSurface::new(entry, instance);
        macos_surface_loader.create_mac_os_surface_mvk(&create_info, None)
    }
    fn choose_phyical_device(instance : &Instance , surface_loader : &Surface , surface : &vk::SurfaceKHR)->vk::PhysicalDevice{
        let devices = unsafe{instance.enumerate_physical_devices()}.expect("No devices available for vulkan.");
        let mut prefered_device = None;
        for physical_device in devices{
            let device_type = unsafe{instance.get_physical_device_properties(physical_device)}.device_type;
            for (queue_index,properties) in unsafe{instance.get_physical_device_queue_family_properties(physical_device)}.iter().enumerate(){
                if prefered_device.is_none() && unsafe{surface_loader.get_physical_device_surface_support(physical_device, queue_index as u32, *surface)}.expect("Failed to query surface support.") && properties.queue_flags.contains(vk::QueueFlags::GRAPHICS){
                    prefered_device = Some(physical_device);
                }
                else if unsafe{surface_loader.get_physical_device_surface_support(physical_device, queue_index as u32, *surface)}.expect("Failed to query surface support.") && properties.queue_flags.contains(vk::QueueFlags::GRAPHICS) && device_type == vk::PhysicalDeviceType::DISCRETE_GPU{
                    prefered_device = Some(physical_device);
                }
            }
        };
        return prefered_device.unwrap();
    }
    fn choose_format(formats : Vec<vk::SurfaceFormatKHR>) -> vk::SurfaceFormatKHR{
        for &format in formats.iter(){
            if format.format == vk::Format::B8G8R8A8_SRGB && format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR{ return format; }
        }
        return formats[1];
    }
    pub fn print_debug_info(&self){
        let device_info = unsafe{self.instance.get_physical_device_properties(self.physical_device)};
        let device_type = match device_info.device_type {
            vk::PhysicalDeviceType::CPU => "Cpu",
            vk::PhysicalDeviceType::INTEGRATED_GPU => "Integrated GPU",
            vk::PhysicalDeviceType::DISCRETE_GPU => "Discrete GPU",
            vk::PhysicalDeviceType::VIRTUAL_GPU => "Virtual GPU",
            vk::PhysicalDeviceType::OTHER => "Unknown",
            _ => panic!(),
        };
        let device_name = Self::vk_to_string(&device_info.device_name);
        println!("Using gpu:{}, of type:{}.",device_name,device_type);
    }
    pub fn vk_to_string(raw_string_array: &[std::os::raw::c_char]) -> String{
        let raw_string = unsafe {
            let pointer = raw_string_array.as_ptr();
            std::ffi::CStr::from_ptr(pointer)
        };
        raw_string
            .to_str()
            .expect("Failed to convert vulkan raw string.")
            .to_owned()
    }
}
impl Drop for VulkanInterface{
    fn drop(&mut self){
        unsafe{self.device.device_wait_idle()}.expect("The renderer crashed.");
        for &framebuffer in self.framebuffers.iter(){
            unsafe{self.device.destroy_framebuffer(framebuffer, None)};
        }
        unsafe{self.device.destroy_render_pass(self.render_pass, None)};
        for &image_view in self.swapchain_image_views.iter(){
            unsafe{self.device.destroy_image_view(image_view, None)}
        }
        unsafe{self.swapchain_loader.destroy_swapchain(self.swapchain, None)};
        unsafe{self.device.destroy_device(None)}
        unsafe{self.surface_loader.destroy_surface(self.surface, None)};
    }
}