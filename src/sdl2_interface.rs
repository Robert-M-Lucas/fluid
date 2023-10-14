use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};
use sdl2::render::WindowCanvas;
use sdl2::video::Window;
use sdl2::{EventPump, TimerSubsystem};

pub struct SDL2Data {
    // pub canvas: WindowCanvas,
    pub renderer: WindowCanvas,
    pub event_pump: EventPump,
    pub timer: TimerSubsystem,
}

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

pub fn init_sdl2() -> SDL2Data {
    let sdl_context = sdl2::init().expect("SDL2 failed to load");
    let mut event_pump = sdl_context.event_pump().expect("Failed to get event pump");
    let timer_subsystem = sdl_context.timer().unwrap();

    let video = sdl_context.video().expect("Failed to get SDL video");
    let gl_attr = video.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let window = video
        .window("Fluid", SCREEN_WIDTH, SCREEN_HEIGHT)
        .opengl()
        .position_centered()
        .build()
        .expect("Failed to create window");
    // let mut canvas = window.into_canvas().build().expect("Failed to convert window to canvas");

    let renderer = window
        .into_canvas()
        .index(find_sdl_gl_driver().unwrap())
        .present_vsync()
        .build()
        .unwrap();

    // let _gl_context = renderer.window().gl_create_context().unwrap();
    renderer.window().gl_set_context_to_current().unwrap();
    let _gl = gl::load_with(|s| video.gl_get_proc_address(s) as *const std::os::raw::c_void);

    SDL2Data {
        // canvas,
        renderer,
        event_pump,
        timer: timer_subsystem,
    }
}
