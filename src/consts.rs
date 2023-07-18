use graphics::types::Color;


pub const WORLD_HEIGHT: usize = 70;
pub const WORLD_WIDTH: usize = 70;
pub const BLOCK_SIZE: f64 = 8.0;
pub const WIN_WIDTH: f64 = WORLD_WIDTH as f64 * BLOCK_SIZE;
pub const WIN_HEIGHT: f64 = WORLD_HEIGHT as f64 * BLOCK_SIZE;

pub const HEAD_COLOR: Color = [0.0, 1.0, 0.0, 1.0];
pub const BODY_COLOR: Color = [0.0, 0.8, 0.0, 1.0];
pub const WALL_COLOR: Color = [0.8, 0.8, 0.8, 1.0];
pub const VOID_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
pub const APPLE_COLOR: Color = [1.0, 0.0, 0.0, 1.0];

pub const UPS: u64 = 15;

