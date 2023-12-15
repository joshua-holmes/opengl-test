use beryllium::{init, video, Sdl, events::Event, error::SdlError};

pub struct Window {
    pub gl_window: video::GlWindow,
    pub sdl: Sdl,
}

impl Window {
    pub fn new(
        window_title: &str,
        window_width: i32,
        window_height: i32,
    ) -> Self {
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

        let gl_window = sdl.create_gl_window(win_args).expect("big sad, no window");

        Self { gl_window, sdl }
    }

    pub fn poll_events(&self) -> Option<(Event, u32)> {
        self.sdl.poll_events()
    }

    pub fn set_swap_interval(&self, interval: video::GlSwapInterval) -> Result<(), SdlError> {
        self.gl_window.set_swap_interval(interval)
    }

    pub fn swap_window(&self) {
        self.gl_window.swap_window();
    }
}

