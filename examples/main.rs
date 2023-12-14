#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use beryllium::{
    Sdl,
    init,
    video,
    events::Event,
};

const WINDOW_TITLE: &str = "My triangle";

fn main() {
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
        title: WINDOW_TITLE,
        width: 800,
        height: 600,
        allow_high_dpi: true,
        borderless: false,
        resizable: false,
    };

    let _win = sdl.create_gl_window(win_args).expect("big sad, no window");

    'main_loop: loop {
        // handle events this frame
        while let Some((event, _timestamp)) = sdl.poll_events() {
            match event {
                Event::Quit => break 'main_loop,
                _ => (),
            }
        }
        // now the events are clear

        // here's where we could change the world state and draw
    }
}
