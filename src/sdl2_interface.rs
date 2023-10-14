use sdl2::{EventPump, TimerSubsystem};
use sdl2::render::WindowCanvas;
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub struct SDL2Data {
    pub canvas: WindowCanvas,
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
            .position_centered()
            // .opengl()
            .build().expect("Failed to create window");
    let mut canvas = window.into_canvas().build().expect("Failed to convert window to canvas");

    SDL2Data {
        canvas,
        event_pump,
        timer: timer_subsystem
    }
}