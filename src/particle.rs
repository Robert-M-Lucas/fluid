use cgmath::{Vector2, Zero};
use crate::Fp;

pub struct Particle {
    pub pos: Vector2<Fp>,
    pub vel: Vector2<Fp>,
    pub accel: Vector2<Fp>,
    pub mass: Fp
}

impl Particle {
    pub fn new(pos: Vector2<Fp>, mass: Fp) -> Self {
        Particle { pos, vel: Vector2::zero(), accel: Vector2::zero(), mass }
    }

    pub fn apply_vel(&mut self, delta_time: Fp) { self.pos += self.vel * delta_time; }

    pub fn apply_accel(&mut self, delta_time: Fp) { self.vel += self.accel * delta_time }

    pub fn set_accel(&mut self, new_accel: Vector2<Fp>) { self.accel = new_accel; }
}
