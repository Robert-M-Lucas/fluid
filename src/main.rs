use array_init::array_init;
use cgmath::{Vector2, Zero};
use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;

type Fp = f32;

struct Particle {
    pub pos: Vector2<Fp>,
    pub vel: Vector2<Fp>
}

impl Particle {
    pub fn new(pos: Vector2<Fp>) -> Self {
        Particle { pos, vel: Vector2::zero() }
    }

    pub fn apply_vel(&mut self, time_step: Fp) {
        self.pos += self.vel * time_step;
    }
}

const WIDTH: u32 = 500;
const HEIGHT: u32 = 500;

fn world_to_screen(world_pos: Vector2<Fp>) -> (u32, u32) {
    (
        world_pos.x as u32 + (WIDTH / 2),
        world_pos.y as u32 + (HEIGHT / 2),
    )
}

fn main() {
    let sdl_context = sdl2::init().expect("SDL2 failed to load");
    let mut event_pump = sdl_context.event_pump().expect("Failed to get event pump");

    let video = sdl_context.video().expect("Failed to get SDL video");
    let window =
        video.window("Fluid", 500, 500)
            .position_centered()
            .opengl()
            .build().expect("Failed to create window");
    let mut canvas = window.into_canvas().build().expect("Failed to convert window to canvas");

    let mut rng = rand::thread_rng();

    let mut balls: [Particle; 20] =
        array_init(|_|
            Particle::new(
                Vector2::new(rng.gen_range(-100..100) as Fp, rng.gen_range(-100..100) as Fp),
            )
        );

    'main_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::P), .. } => break 'main_loop,
                _ => {}
            }
        }

        for b in balls {
            let pos = world_to_screen(b.pos);
            canvas.aa_circle(pos.0 as i16, pos.1 as i16, 4, Color::RGB(255, 255, 255)).unwrap();
        }

        canvas.present();
    }
}
