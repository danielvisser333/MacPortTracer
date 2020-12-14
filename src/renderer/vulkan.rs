use ash::Entry;
use ash::version::EntryV1_0;
use ash::Instance;
use ash::vk;
use ash::extensions::khr::Surface;

use std::ffi::CString;

pub struct VulkanInterface{
    entry : Entry,
    instance : Instance,
    surface_loader : Surface,
    surface : vk::SurfaceKHR,
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
        let surface = Self::create_surface(&entry, &instance, &window);
        return Self{
            entry,instance,surface_loader,surface
        }
    }
    #[cfg(all(windows))]
    pub fn create_surface(entry : &Entry , instance : &Instance , window : &winit::window::Window) -> vk::SurfaceKHR{
        use winapi::um::libloaderapi::GetModuleHandleW;
        use winit::platform::windows::WindowExtWindows;
        let hinstance = unsafe{GetModuleHandleW(std::ptr::null())} as *const std::ffi::c_void;
        let hwnd = window.hwnd();
        let create_info = ash::vk::Win32SurfaceCreateInfoKHR{
            s_type: ash::vk::StructureType::WIN32_SURFACE_CREATE_INFO_KHR,
            p_next: std::ptr::null(),
            flags: Default::default(),
            hinstance,
            hwnd: hwnd,
        };
        unsafe{ash::extensions::khr::Win32Surface::new(entry, instance).create_win32_surface(&create_info, None)}.expect("Failed to create Win32 surface.")
    }
    #[cfg(target_os = "macos")]
    pub unsafe fn create_surface<E: EntryV1_0, I: InstanceV1_0>(entry: &E,instance: &I,window: &winit::window::Window,) -> vk::SurfaceKHR {
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
}
impl Drop for VulkanInterface{
    fn drop(&mut self){
        unsafe{self.surface_loader.destroy_surface(self.surface, None)};
    }
}