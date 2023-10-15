use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};
use sdl2::render::WindowCanvas;
use sdl2::{EventPump, TimerSubsystem};
use crate::opengl_interface::{init_opengl, ShaderProgram};

pub struct SDL2Data {
    pub shader_program: ShaderProgram,
    pub renderer: WindowCanvas,
    pub event_pump: EventPump,
    pub timer: TimerSubsystem,
}

pub fn init_sdl2() -> SDL2Data {
    let sdl_context = sdl2::init().expect("SDL2 failed to load");
    let mut event_pump = sdl_context.event_pump().expect("Failed to get event pump");
    let timer_subsystem = sdl_context.timer().unwrap();

    let video = sdl_context.video().expect("Failed to get SDL video");

    let window = video
        .window("Fluid", SCREEN_WIDTH, SCREEN_HEIGHT)
        .opengl()
        .position_centered()
        .build()
        .expect("Failed to create window");
    // let mut canvas = window.into_canvas().build().expect("Failed to convert window to canvas");

    let (renderer, shader_program) = init_opengl(window, &video);

    // let _gl_context = renderer.window().gl_create_context().unwrap();

    SDL2Data {
        shader_program,
        renderer,
        event_pump,
        timer: timer_subsystem,
    }
}

