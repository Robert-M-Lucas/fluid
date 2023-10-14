use std::cell::RefCell;
use array_init::array_init;
use cgmath::Vector2;
use rand::{Rng, thread_rng};
use rand::rngs::ThreadRng;
use crate::particle::Particle;
use crate::{Fp, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::math::screen_to_world;

pub enum SpawningMethod {
    Random
}

impl SpawningMethod {
    pub fn get_particles<const ParticleCount: usize>(&self) -> [Particle; ParticleCount] {
        let mut rng = thread_rng();
        array_init(|_|
            Particle::new(
                screen_to_world((
                    rng.gen_range(0..SCREEN_WIDTH),
                    rng.gen_range(0..SCREEN_HEIGHT),
                )),
                1.0
            )
        )
    }
}

pub struct SceneData<const C: usize> {
    pub particles: [Particle; C],
    pub rng: ThreadRng
}

impl<const C: usize> SceneData<C> {
    pub fn new(particle_spawning_method: SpawningMethod) -> SceneData<C> {
        SceneData {
            particles: particle_spawning_method.get_particles(),
            rng: thread_rng()
        }
    }
}