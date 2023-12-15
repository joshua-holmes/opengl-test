use beryllium::{init, video, Sdl};

pub fn create_gl_window_obj(
    window_title: &str,
    window_width: i32,
    window_height: i32,
) -> video::GlWindow {
    let sdl = Sdl::init(init::InitFlags::EVERYTHING);
    sdl.set_gl_context_major_version(3).unwrap();
    sdl.set_gl_profile(video::GlProfile::Core).unwrap();
    let mut flags = video::GlContextFlags::default();
    #[cfg(target_os = "macos")]
    {
        flags |= video::GlContextFlags::FORWARD_COMPATIBLE;
    }
    if cfg!(debug_asserts) {
        flags |= video::GlContextFlags::DEBUG;
    }
    sdl.set_gl_context_flags(flags).unwrap();
    let win_args = video::CreateWinArgs {
        title: window_title,
        width: window_width,
        height: window_height,
        allow_high_dpi: true,
        borderless: false,
        resizable: false,
    };

    sdl.create_gl_window(win_args).expect("big sad, no window")
}
