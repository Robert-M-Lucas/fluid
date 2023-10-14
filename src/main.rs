use cgmath::{Vector2, Zero};
use gl::types::GLfloat;
use rand::Rng;
use sdl2::event::Event;
use sdl2::gfx::framerate::FPSManager;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::TimerSubsystem;
use crate::math::screen_to_world;
use crate::physics::physics_update;
use crate::renderer::render_scene_data;
use crate::scene_data::{SceneData, SpawningMethod};
use crate::sdl2_interface::init_sdl2;

mod math;
mod particle;
mod scene_data;
mod renderer;
mod sdl2_interface;
mod physics;

pub type Fp = f32;

pub const SCREEN_WIDTH: u32 = 500;
pub const SCREEN_HEIGHT: u32 = 500;

pub const WORLD_HEIGHT: Fp = 1.0; // Screen height in metres
pub const WORLD_WIDTH: Fp = (SCREEN_WIDTH as Fp / SCREEN_HEIGHT as Fp) * WORLD_HEIGHT;

pub const PARTICLE_COUNT: usize = 1000;

pub const TARGET_FPS: u32 = 200;

pub const CURSOR_FORCE: Fp = 15.0;
pub const CURSOR_RADIUS: Fp = 0.2;

pub enum CursorState {
    Push(Vector2<Fp>),
    Pull(Vector2<Fp>),
    None
}

fn main() {
    let mut sdl2_data = init_sdl2();
    let mut scene_data = SceneData::<PARTICLE_COUNT>::new(SpawningMethod::Random);
    let mut fps_manager = FPSManager::new();
    fps_manager.set_framerate(TARGET_FPS).unwrap();



    unsafe {
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    let delta_time = 1.0 / TARGET_FPS as Fp;

    let mut prev_tick = sdl2_data.timer.performance_counter();
    let tick_freq = sdl2_data.timer.performance_frequency();

    let mut frame: u128 = 0;

    'main_loop: loop {
        for event in sdl2_data.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::P), .. } => break 'main_loop,
                _ => {}
            }
        }

        let cursor_state =
            if sdl2_data.event_pump.mouse_state().left() {
                CursorState::Push(
                    screen_to_world(
                        (
                            sdl2_data.event_pump.mouse_state().x() as u32,
                            sdl2_data.event_pump.mouse_state().y() as u32,
                        )
                    )
                )
            }
            else if sdl2_data.event_pump.mouse_state().right() {
                CursorState::Pull(
                    screen_to_world(
                        (
                            sdl2_data.event_pump.mouse_state().x() as u32,
                            sdl2_data.event_pump.mouse_state().y() as u32,
                        )
                    )
                )
            }
            else { CursorState::None };

        let tick = sdl2_data.timer.performance_counter();
        let true_delta_time = (tick - prev_tick) as Fp / tick_freq as Fp;
        prev_tick = tick;


        physics_update(&mut scene_data, true_delta_time, &cursor_state);

        // sdl2_data.canvas.set_draw_color(Color::BLACK);
        // sdl2_data.canvas.clear();

        render_scene_data(&scene_data, &mut sdl2_data);

        // sdl2_data.canvas.present();

        // fps_manager.delay();

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        sdl2_data.window.gl_swap_window();

        if frame % TARGET_FPS as u128 == 0 {
            println!("{} fps", 1.0 / true_delta_time);
        }

        frame += 1;
    }
}
