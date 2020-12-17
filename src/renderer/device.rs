use ash::vk::PhysicalDevice;

use ash::extensions::khr::Surface;
use ash::vk::SurfaceKHR;

use ash::Instance;
use ash::version::InstanceV1_0;

use ash::Device;

pub fn choose_physical_device(instance : &Instance , surface_loader : &Surface , surface : &SurfaceKHR) -> PhysicalDevice{
    let physical_devices = unsafe{instance.enumerate_physical_devices()}.expect("No devices that support vulkan found.");
    let mut prefered_device = None;
    for &device in physical_devices.iter(){
        let device_type = unsafe{instance.get_physical_device_properties(device).device_type};
        for (queue_index,properties) in unsafe{instance.get_physical_device_queue_family_properties(device)}.iter().enumerate(){
            if prefered_device.is_none() && unsafe{surface_loader.get_physical_device_surface_support(device, queue_index as u32, *surface)}.expect("Failed to query surface support.") && properties.queue_flags.contains(ash::vk::QueueFlags::GRAPHICS){
                prefered_device = Some(device);
            }
            else if unsafe{surface_loader.get_physical_device_surface_support(device, queue_index as u32, *surface)}.expect("Failed to query device surface support.") && properties.queue_flags.contains(ash::vk::QueueFlags::GRAPHICS) && device_type == ash::vk::PhysicalDeviceType::DISCRETE_GPU{
                prefered_device = Some(device);
            }
        }
    }
    return prefered_device.unwrap();
}
pub fn get_graphics_queue_family(instance : &Instance , physical_device : &PhysicalDevice) -> u32{
    let queue_families = unsafe{instance.get_physical_device_queue_family_properties(*physical_device)};
    let mut queue_family_index = None;
    for (index,queue_family) in queue_families.iter().enumerate(){
        if queue_family_index.is_none() && queue_family.queue_flags.contains(ash::vk::QueueFlags::GRAPHICS){
            queue_family_index = Some(index as u32);
        }
    }
    return queue_family_index.unwrap();
}
pub fn get_presentation_queue_family(instance : &Instance , physical_device : &PhysicalDevice , surface_loader : &Surface , surface : &SurfaceKHR) -> u32{
    let queue_families = unsafe{instance.get_physical_device_queue_family_properties(*physical_device)};
    let mut queue_family_index = None;
    for (index,_) in queue_families.iter().enumerate(){
        if unsafe{surface_loader.get_physical_device_surface_support(*physical_device, index as u32, *surface)}.expect("Failed to query presentation support."){
            queue_family_index = Some(index as u32);
            break;
        }
    }
    return queue_family_index.unwrap();
}
pub fn get_transfer_queue_family(instance : &Instance , physical_device : &PhysicalDevice) -> u32{
    let queue_families = unsafe{instance.get_physical_device_queue_family_properties(*physical_device)};
    let mut queue_family_index = None;
    for (index,queue_family) in queue_families.iter().enumerate(){
        if queue_family_index.is_none() && queue_family.queue_flags.contains(ash::vk::QueueFlags::TRANSFER){
            queue_family_index = Some(index as u32);
        } else if queue_family.queue_flags.contains(ash::vk::QueueFlags::TRANSFER) && !queue_family.queue_flags.contains(ash::vk::QueueFlags::GRAPHICS) && !queue_family.queue_flags.contains(ash::vk::QueueFlags::COMPUTE){
            queue_family_index = Some(index as u32);
        }
    }
    return queue_family_index.unwrap();
}
pub fn get_compute_queue_family(instance : &Instance , physical_device : &PhysicalDevice) -> u32{
    let queue_families = unsafe{instance.get_physical_device_queue_family_properties(*physical_device)};
    let mut queue_family_index = None;
    for (index,queue_family) in queue_families.iter().enumerate(){
        if queue_family_index.is_none() && queue_family.queue_flags.contains(ash::vk::QueueFlags::COMPUTE){
            queue_family_index = Some(index as u32);
        } else if queue_family.queue_flags.contains(ash::vk::QueueFlags::COMPUTE) && !queue_family.queue_flags.contains(ash::vk::QueueFlags::GRAPHICS){
            queue_family_index = Some(index as u32);
        }
    }
    return queue_family_index.unwrap();
}
pub fn create_device(instance : &Instance , physical_device : &PhysicalDevice, graphics_queue_family : u32, transfer_queue_family : u32, compute_queue_family : u32, presentation_queue_family : u32) -> Device{
    let mut queues = vec!(graphics_queue_family,transfer_queue_family,compute_queue_family,presentation_queue_family);
    queues.sort();
    queues.dedup();
    let mut queue_infos = Vec::new();
    let priorities = [1.0f32];
    for queue in queues{
        queue_infos.push(ash::vk::DeviceQueueCreateInfo{
            s_type : ash::vk::StructureType::DEVICE_QUEUE_CREATE_INFO,
            p_next : std::ptr::null(),
            flags : ash::vk::DeviceQueueCreateFlags::empty(),
            queue_count : 1,
            queue_family_index : queue,
            p_queue_priorities : priorities.as_ptr(),
        })
    }
    let extensions = [ash::extensions::khr::Swapchain::name().as_ptr()];
    let features = ash::vk::PhysicalDeviceFeatures{
        ..Default::default()
    };
    let device_create_info = ash::vk::DeviceCreateInfo{
        s_type : ash::vk::StructureType::DEVICE_CREATE_INFO,
        p_next : std::ptr::null(),
        flags : ash::vk::DeviceCreateFlags::empty(),
        enabled_extension_count : extensions.len() as u32,
        pp_enabled_extension_names : extensions.as_ptr(),
        enabled_layer_count : 0,
        pp_enabled_layer_names : std::ptr::null(),
        queue_create_info_count : queue_infos.len() as u32,
        p_queue_create_infos : queue_infos.as_ptr(),
        p_enabled_features : &features,
    };
    return unsafe{instance.create_device(*physical_device, &device_create_info, None)}.expect("Failed to create logical device.");
}