use crate::math::world_to_screen;
use crate::scene_data::SceneData;
use crate::sdl2_interface::SDL2Data;
use cgmath::InnerSpace;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;

pub fn render_scene_data<const C: usize>(scene_data: &SceneData<C>, sdl2_data: &mut SDL2Data) {
    for particle in &scene_data.particles {
        let pos = world_to_screen(particle.pos);

        let mut vel = particle.vel.magnitude();
        if vel > 0.6 {
            vel = 0.6
        }

        let red = (vel * (255.0)) as u8;

        let colour = Color::RGB(red, 0, 255 - red);

        // sdl2_data.canvas.filled_circle(pos.0, pos.1, 8, colour).unwrap();
    }
}
