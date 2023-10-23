#![allow(dead_code)]

use crate::math::{generate_triangle, screen_to_world, world_to_open_gl};
use crate::physics::physics_update;
use crate::renderer::render_scene_data;
use crate::scene_data::{SceneData, SpawningMethod};
use crate::sdl2_interface::init_sdl2;
use cgmath::{InnerSpace, Vector2, Zero};
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
mod opengl_interface;

pub type Fp = f32;

pub const SCREEN_WIDTH: u32 = 1000;
pub const SCREEN_HEIGHT: u32 = 1000;

pub const WORLD_HEIGHT: Fp = 0.5; // Screen height in metres
pub const WORLD_WIDTH: Fp = (SCREEN_WIDTH as Fp / SCREEN_HEIGHT as Fp) * WORLD_HEIGHT;

pub const PARTICLE_COUNT: usize = 1000;

pub const TARGET_FPS: u32 = 10;

pub const CURSOR_FORCE: Fp = 3.0;
pub const CURSOR_RADIUS: Fp = 0.3;

pub const USE_TRUE_DELTA_TIME: bool = false;

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
    let delta_time = 1.0 / 100.0;

    let mut prev_tick = sdl2_data.timer.performance_counter();
    let tick_freq = sdl2_data.timer.performance_frequency();

    let mut frame: u128 = 0;

    unsafe {
        gl::Viewport(0, 0, SCREEN_WIDTH as GLsizei, SCREEN_HEIGHT as GLsizei);
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
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

        if USE_TRUE_DELTA_TIME {
            physics_update(&mut scene_data, true_delta_time, &cursor_state);
        }
        else {
            physics_update(&mut scene_data, delta_time, &cursor_state);
        }

        // render_scene_data(&scene_data, &mut sdl2_data);

        // sdl2_data.canvas.present();


        // Initialise vertices for triangle
        let mut vertices: Vec<f32> = Vec::with_capacity(6 * PARTICLE_COUNT);

        for particle in &scene_data.particles {
            let mut vel = particle.vel.magnitude();
            if vel > 0.6 {
                vel = 0.6
            }

            let red = (vel * (1.0 / 0.6)).sqrt();
            // let red = red * red;

            let vel = vel.clamp(0.0, 0.5);
            let mut first = true;
            let triangles = if vel > 0.2 {
                let vel = ((vel - 0.2) / 2.0) + 0.2;
                generate_triangle((particle.vel.normalize() * vel) / 25.0)
            }
            else {
                first = false;
                generate_triangle((particle.vel.normalize() * 0.2) / 25.0)
            };

            for mut offset in triangles {
                if first {
                    first = false;
                }
                else {
                    offset = offset * 0.6;
                }
                let pos = world_to_open_gl(Vector2::new( particle.pos.x + offset.x, particle.pos.y + offset.y));
                vertices.push(pos.x);
                vertices.push(pos.y);
                vertices.push(0.0);

                vertices.push(red);
                vertices.push(0.0);
                vertices.push(1.0 - red);
            }
        }


        let mut vbo: gl::types::GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo); // Request 1 buffer, put buffer name into vbo
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo); // Binds named buffer to target
            gl::BufferData(
                gl::ARRAY_BUFFER, // Target
                (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // Size of data
                vertices.as_ptr() as *const gl::types::GLvoid, // Pointer to data
                gl::STATIC_DRAW, // Usage
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0); // Unbind the buffer
        }

        // Instructions for how to interpret vertices
        let mut vao: gl::types::GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao); // Request 1 buffer, put name into vao
            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
            gl::VertexAttribPointer(
                0, // index of the generic vertex attribute ("layout (location = 0)")
                3, // the number of components per generic vertex attribute
                gl::FLOAT, // data type
                gl::FALSE, // normalized (int-to-float conversion)
                (6 * std::mem::size_of::<f32>()) as gl::types::GLint, // Stride (byte offset between consecutive attributes) - here 3 * f32 for x, y, z
                std::ptr::null() // offset of the first component
            );
            gl::EnableVertexAttribArray(1); // this is "layout (location = 1)" in vertex shader
            gl::VertexAttribPointer(
                1, // index of the generic vertex attribute ("layout (location = 1)")
                3, // the number of components per generic vertex attribute
                gl::FLOAT, // data type
                gl::FALSE, // normalized (int-to-float conversion)
                (6 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
                (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid // offset of the first component
            );

            gl::BindBuffer(gl::ARRAY_BUFFER, 0); // Unbind buffer
            gl::BindVertexArray(0); // Unbind vertex array
        }

        unsafe { gl::BindVertexArray(vao); }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }


        unsafe {
            gl::DrawArrays(
                gl::TRIANGLES, // mode
                0, // starting index in the enabled arrays
                (vertices.len() / 2) as i32 // number of indices to be rendered
            );
        }

        sdl2_data.renderer.window().gl_swap_window();

        fps_manager.delay();

        if frame % TARGET_FPS as u128 == 0 {
            println!("{} fps", 1.0 / true_delta_time);
        }

        frame += 1;
    }
}
