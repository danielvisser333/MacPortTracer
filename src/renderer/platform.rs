#[cfg(all(windows))]
pub fn create_surface(entry : &ash::Entry , instance : &ash::Instance , window : &winit::window::Window) -> ash::vk::SurfaceKHR{
    use winapi::um::libloaderapi::GetModuleHandleW;
    use winit::platform::windows::WindowExtWindows;
    let hinstance = unsafe{GetModuleHandleW(std::ptr::null())} as *const std::ffi::c_void;
    let hwnd = window.hwnd();
    let create_info = ash::vk::Win32SurfaceCreateInfoKHR{
        s_type: ash::vk::StructureType::WIN32_SURFACE_CREATE_INFO_KHR,
        p_next: std::ptr::null(),
        flags: Default::default(),
        hinstance,
        hwnd,
    };
    return unsafe{ash::extensions::khr::Win32Surface::new(entry, instance).create_win32_surface(&create_info, None)}.expect("Failed to create Win32 surface.");
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