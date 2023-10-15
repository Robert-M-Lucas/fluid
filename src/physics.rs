use crate::scene_data::SceneData;
use crate::{CursorState, Fp, CURSOR_FORCE, CURSOR_RADIUS, WORLD_HEIGHT, WORLD_WIDTH};
use cgmath::{InnerSpace, Vector2, Zero};
use rand::Rng;

// const GRAVITY: Fp = -9.81;
const GRAVITY: Fp = 0.0;
const COEF_OF_REST: Fp = 0.1;
const DRAG_COEF: Fp = 1.0;
const PARTICLE_FORCE_SCALE: Fp = 0.0002;
const WALL_FORCE_SCALE: Fp = 0.005;

pub fn physics_update<const C: usize>(
    scene_data: &mut SceneData<C>,
    delta_time: Fp,
    cursor_state: &CursorState,
) {
    // ! If removed, must be replaced with p.accel = 0 !
    scene_data
        .particles
        .iter_mut()
        .for_each(|p| p.accel = Vector2::new(0.0, GRAVITY));
    scene_data
        .particles
        .iter_mut()
        .for_each(|p| p.accel += (-p.vel * DRAG_COEF) / p.mass);

    match cursor_state {
        CursorState::Pull(pos) => scene_data.particles.iter_mut().for_each(|p| {
            if (pos - p.pos).magnitude() < CURSOR_RADIUS {
                p.accel += (pos - p.pos).normalize() * CURSOR_FORCE
            }
        }),
        CursorState::Push(pos) => scene_data.particles.iter_mut().for_each(|p| {
            if (pos - p.pos).magnitude() < CURSOR_RADIUS {
                p.accel += -(pos - p.pos).normalize() * CURSOR_FORCE
            }
        }),
        CursorState::None => {}
    }

    apply_repulsive_particle_force(scene_data);

    apply_repulsive_wall_force(scene_data);

    scene_data
        .particles
        .iter_mut()
        .for_each(|p| p.apply_accel(delta_time));
    scene_data
        .particles
        .iter_mut()
        .for_each(|p| p.apply_vel(delta_time));

    bound_particles(scene_data, delta_time);
}

pub fn get_force(pos1: Vector2<Fp>, pos2: Vector2<Fp>, force_scale: Fp) -> Vector2<Fp> {
    let mut displacement = pos2 - pos1; // 1 to 2
    if displacement == Vector2::<Fp>::zero() {
        displacement = Vector2::new(0.01, 0.01);
    }

    let mut distance = displacement.magnitude();
    if distance < 0.01 {
        distance = 0.01;
    }
    let direction = displacement.normalize();
    let force = (1.0 / (distance * distance)) * force_scale;
    -direction * force
}

pub fn apply_repulsive_particle_force<const C: usize>(scene_data: &mut SceneData<C>) {
    for i in 0..C {
        for j in 0..C {
            if i == j {
                continue;
            }
            let force = get_force(scene_data.particles[i].pos, scene_data.particles[j].pos, PARTICLE_FORCE_SCALE);
            scene_data.particles[i].accel += force / scene_data.particles[i].mass;
            scene_data.particles[j].accel += -force / scene_data.particles[j].mass;
        }
    }
}

pub fn apply_repulsive_wall_force<const C: usize>(scene_data: &mut SceneData<C>) {
    for particle in &mut scene_data.particles {
        let pos = particle.pos;

        let mut total_force = Vector2::zero();
        total_force += get_force(pos, Vector2::new(-0.01, pos.y), WALL_FORCE_SCALE);
        total_force += get_force(pos, Vector2::new(WORLD_WIDTH + 0.01, pos.y), WALL_FORCE_SCALE);
        total_force += get_force(pos, Vector2::new(pos.x, -0.01), WALL_FORCE_SCALE);
        total_force += get_force(pos, Vector2::new(pos.x, WORLD_HEIGHT + 0.01), WALL_FORCE_SCALE);
        particle.accel += total_force / particle.mass;
    }
}

pub fn bound_particles<const C: usize>(scene_data: &mut SceneData<C>, delta_time: Fp) {
    for particle in &mut scene_data.particles {
        // for (pos, vel, accel) in [(&mut particle.pos.x, &mut particle.vel.x, &mut particle.accel.x), (&mut particle.pos.y, &mut particle.vel.y, &mut particle.accel.y)] {
        //     if *pos >= 0.0 { continue; }
        //     // let dist_moved = *vel * delta_time;
        //     // let dist_illegal = -*pos;
        //     // let proportion =  (dist_illegal / dist_moved).abs();
        //     *vel = -*vel;
        //     // *vel += *accel * delta_time * proportion;
        //     *pos = -*pos;
        // }

        if particle.pos.x < 0.0 {
            particle.pos.x = -particle.pos.x;
            particle.vel.x = -particle.vel.x * COEF_OF_REST;
        }
        if particle.pos.y < 0.0 {
            particle.pos.y = -particle.pos.y;
            particle.vel.y = -particle.vel.y * COEF_OF_REST;
        }
        if particle.pos.x > WORLD_WIDTH {
            particle.pos.x = WORLD_WIDTH - (particle.pos.x - WORLD_WIDTH);
            particle.vel.x = -particle.vel.x * COEF_OF_REST;
        }
        if particle.pos.y > WORLD_HEIGHT {
            particle.pos.y = WORLD_HEIGHT - (particle.pos.y - WORLD_HEIGHT);
            particle.vel.y = -particle.vel.y * COEF_OF_REST;
        }
    }
}
