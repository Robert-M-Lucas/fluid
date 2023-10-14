use crate::{Fp, SCREEN_HEIGHT, SCREEN_WIDTH, WORLD_HEIGHT};
use cgmath::{Vector2, Vector3};

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
