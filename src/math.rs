use crate::{Fp, SCREEN_HEIGHT, SCREEN_WIDTH, WORLD_HEIGHT, WORLD_WIDTH};
use cgmath::{Vector2, Vector3, Rotation};
use cgmath::num_traits::FloatConst;

const WORLD_TO_SCREEN_SCALE_FACTOR: Fp = SCREEN_HEIGHT as Fp / WORLD_HEIGHT;

pub fn world_to_screen(world_pos: Vector2<Fp>) -> (i16, i16) {
    (
        (world_pos.x * WORLD_TO_SCREEN_SCALE_FACTOR) as i16,
        SCREEN_HEIGHT as i16 - (world_pos.y * WORLD_TO_SCREEN_SCALE_FACTOR) as i16,
    )
}

pub fn screen_to_world(screen_pos: (u32, u32)) -> Vector2<Fp> {
    Vector2::new(
        screen_pos.0 as Fp / WORLD_TO_SCREEN_SCALE_FACTOR,
        (SCREEN_HEIGHT - screen_pos.1) as Fp / WORLD_TO_SCREEN_SCALE_FACTOR,
    )
}

pub fn world_to_open_gl(world_pos: Vector2<Fp>) -> Vector2<Fp> {
    let (x, y) = world_pos.into();
    let (x, y) = (2.0 * x / WORLD_WIDTH, 2.0 * y / WORLD_HEIGHT); // Normalise x and y between 0 and 1
    let (x, y) = (x - 1.0, y - 1.0); // Adjust x and y to be between -1 and 1
    Vector2::new(x, y)
}

pub fn generate_triangle(vector: Vector2<Fp>) -> [Vector2<Fp>; 3] {
    let sixty = (2.0 * Fp::PI()) / 3.0;
    let vector_two = rotate_vector(&vector, sixty);
    let vector_three = rotate_vector(&vector, sixty * 2.0);
    [vector, vector_two, vector_three]
}

pub fn rotate_vector(vector: &Vector2<Fp>, angle_rad: Fp) -> Vector2<Fp> {
    Vector2::new(angle_rad.cos() * vector.x - angle_rad.sin() * vector.y, angle_rad.sin() * vector.x + angle_rad.cos() * vector.y)
}