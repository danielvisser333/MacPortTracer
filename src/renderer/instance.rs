use ash::Entry;
use ash::version::EntryV1_0;
use ash::Instance;

use ash::vk;

use std::ffi::CString;

pub fn create_entry() -> Entry{
    return Entry::new().expect("Vulkan is not supported on your device.");
}
pub fn create_instance(entry : &Entry) -> Instance{
    let mut exts = vec!(); 
    exts.push(ash::extensions::khr::Surface::name().as_ptr());
    #[cfg(all(windows))]
    exts.push(ash::extensions::khr::Win32Surface::name().as_ptr());
    #[cfg(target_os = "macos")]
    exts.push(ash::extensions::mvk::MacOSSurface::name().as_ptr());
    #[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
    exts.push(ash::extensions::khr::XlibSurface::name().as_ptr());
    let application_info = vk::ApplicationInfo{
        s_type : vk::StructureType::APPLICATION_INFO,
        p_next : std::ptr::null(),
        api_version : vk::make_version(1, 2, 0),
        p_application_name : CString::new(super::super::APP_NAME).unwrap().as_ptr(),
        p_engine_name : CString::new(super::super::ENGINE_NAME).unwrap().as_ptr(),
        application_version : *super::super::VERSION,
        engine_version : *super::super::VERSION,
    };
    let validation = if *super::super::VALIDATION_ENABLED{
        vec!(CString::new("VK_LAYER_KHRONOS_validation").unwrap())
    } else{
        vec!()
    };
    let validation_raw : Vec<*const i8> = validation.iter().map(|name| name.as_ptr()).collect();
    let instance_create_info = vk::InstanceCreateInfo{
        s_type : vk::StructureType::INSTANCE_CREATE_INFO,
        p_next : std::ptr::null(),
        flags : vk::InstanceCreateFlags::empty(),
        p_application_info : &application_info,
        pp_enabled_extension_names : exts.as_ptr(),
        enabled_extension_count : exts.len() as u32,
        enabled_layer_count : if *super::super::VALIDATION_ENABLED{1}else{0},
        pp_enabled_layer_names : validation_raw.as_ptr(),
    };
    return unsafe{entry.create_instance(&instance_create_info,None)}.expect("Failed to create vulkan instance, are your drivers up to date?");
}