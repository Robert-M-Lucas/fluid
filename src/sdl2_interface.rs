use sdl2::{EventPump, TimerSubsystem};
use sdl2::render::WindowCanvas;
use sdl2::video::Window;
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub struct SDL2Data {
    // pub canvas: WindowCanvas,
    pub window: Window,
    pub event_pump: EventPump,
    pub timer: TimerSubsystem
}

pub fn init_sdl2() -> SDL2Data {
    let sdl_context = sdl2::init().expect("SDL2 failed to load");
    let mut event_pump = sdl_context.event_pump().expect("Failed to get event pump");
    let timer_subsystem = sdl_context.timer().unwrap();

    let video = sdl_context.video().expect("Failed to get SDL video");
    let window =
        video.window("Fluid", SCREEN_WIDTH, SCREEN_HEIGHT)
            .opengl()
            .position_centered()
            .build().expect("Failed to create window");
    // let mut canvas = window.into_canvas().build().expect("Failed to convert window to canvas");

    let _gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video.gl_get_proc_address(s) as *const std::os::raw::c_void);

    SDL2Data {
        // canvas,
        window,
        event_pump,
        timer: timer_subsystem
    }
}