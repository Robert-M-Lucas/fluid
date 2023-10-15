#![allow(dead_code)]

use crate::math::screen_to_world;
use crate::physics::physics_update;
use crate::renderer::render_scene_data;
use crate::scene_data::{SceneData, SpawningMethod};
use crate::sdl2_interface::init_sdl2;
use cgmath::{Vector2, Zero};
use gl::types::{GLfloat, GLsizei};
use rand::Rng;
use sdl2::event::Event;
use sdl2::gfx::framerate::FPSManager;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::TimerSubsystem;

mod math;
mod particle;
mod physics;
mod renderer;
mod scene_data;
mod sdl2_interface;

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
    None,
}

fn main() {
    let mut sdl2_data = init_sdl2();
    let mut scene_data = SceneData::<PARTICLE_COUNT>::new(SpawningMethod::Random);
    let mut fps_manager = FPSManager::new();
    fps_manager.set_framerate(TARGET_FPS).unwrap();

    let delta_time = 1.0 / TARGET_FPS as Fp;

    let mut prev_tick = sdl2_data.timer.performance_counter();
    let tick_freq = sdl2_data.timer.performance_frequency();

    let mut frame: u128 = 0;

    unsafe {
        gl::ClearColor(0.0, 1.0, 0.0, 1.0);
    }

    let vertices: Vec<f32> = vec![
        -0.5, -0.5, 0.0,
        0.5, -0.5, 0.0,
        0.0, 0.5, 0.5
    ];

    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
    }

    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER, // target
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
            vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW, // usage
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind the buffer
    }

    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
    }

    unsafe {
        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
        gl::VertexAttribPointer(
            0, // index of the generic vertex attribute ("layout (location = 0)")
            3, // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (3 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            std::ptr::null() // offset of the first component
        );

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }
    unsafe {
        gl::BindVertexArray(vao);
    }

    'main_loop: loop {
        for event in sdl2_data.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::P),
                    ..
                } => break 'main_loop,
                _ => {}
            }
        }

        let cursor_state = if sdl2_data.event_pump.mouse_state().left() {
            CursorState::Push(screen_to_world((
                sdl2_data.event_pump.mouse_state().x() as u32,
                sdl2_data.event_pump.mouse_state().y() as u32,
            )))
        } else if sdl2_data.event_pump.mouse_state().right() {
            CursorState::Pull(screen_to_world((
                sdl2_data.event_pump.mouse_state().x() as u32,
                sdl2_data.event_pump.mouse_state().y() as u32,
            )))
        } else {
            CursorState::None
        };

        let tick = sdl2_data.timer.performance_counter();
        let true_delta_time = (tick - prev_tick) as Fp / tick_freq as Fp;
        prev_tick = tick;

        // physics_update(&mut scene_data, true_delta_time, &cursor_state);

        // render_scene_data(&scene_data, &mut sdl2_data);

        // sdl2_data.canvas.present();

        // fps_manager.delay();

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        unsafe {
            gl::DrawArrays(
                gl::TRIANGLES, // mode
                0, // starting index in the enabled arrays
                3 // number of indices to be rendered
            );
        }

        sdl2_data.renderer.window().gl_swap_window();

        if frame % TARGET_FPS as u128 == 0 {
            println!("{} fps", 1.0 / true_delta_time);
        }

        frame += 1;
    }
}
