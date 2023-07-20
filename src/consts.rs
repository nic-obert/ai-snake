use std::time::Duration;
use graphics::types::Color;


// Map constants

pub const MAP_START_Y: f64 = TOPBAR_HEIGHT;
pub const WORLD_HEIGHT: usize = 75;
pub const WORLD_WIDTH: usize = 90;
pub const BLOCK_SIZE: f64 = 8.0;


// Window constants

pub const WIN_TITLE: &str = "Snake";

#[macro_export]
macro_rules! font_path {
    () => {
        "../assets/FiraSans-Regular.ttf"
    };
}

pub const FONT_SIZE: u32 = 24;

pub const TOPBAR_HEIGHT: f64 = 50.0;

pub const WIN_WIDTH: f64 = WORLD_WIDTH as f64 * BLOCK_SIZE;
pub const WIN_HEIGHT: f64 = TOPBAR_HEIGHT + WORLD_HEIGHT as f64 * BLOCK_SIZE;


// Colors

pub const BACKGROUND_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
pub const HEAD_COLOR: Color = [0.0, 1.0, 0.0, 1.0];
pub const TAIL_COLOR: Color = [0.7, 0.8, 0.2, 1.0];
pub const WALL_COLOR: Color = [0.8, 0.8, 0.8, 1.0];
pub const VOID_COLOR: Color = BACKGROUND_COLOR;
pub const APPLE_COLOR: Color = [1.0, 0.0, 0.0, 1.0];
pub const TEXT_COLOR: Color = [1.0, 1.0, 1.0, 1.0];


// Game constants

pub const STEPS_PER_SECOND: f64 = 50.0;
/// The time between each game update.
/// Updated automatically
pub const UPDATE_DEALY: f64 = 1.0 / STEPS_PER_SECOND;

pub const NEXT_GENERATION_DELAY: Duration = Duration::from_millis(0);

// Snake constants

pub const INITIAL_SNAKE_LENGTH: usize = 3;


// AI agent constants

pub const SIGHT_RADIUS: usize = 6;
pub const SIGHT_SIZE: usize = 1 + SIGHT_RADIUS * 2;
/// The number of inputs the snake's brain will receive.
/// Updated automatically
pub const SIGHT_INPUT_SIZE: usize = SIGHT_SIZE * SIGHT_SIZE;

pub const GENERATION_SIZE: usize = 20;
pub const MAX_APPLES: usize = 50;

pub const MUTATION_CHANCE: f64 = 0.8;
pub const MAX_MUTATION: f64 = 0.5;

pub const GENERATION_CARRYOVER: usize = 5;

