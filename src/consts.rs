use graphics::types::Color;


// Map constants

pub const MAP_START_Y: f64 = TOPBAR_HEIGHT;
pub const WORLD_HEIGHT: usize = 70;
pub const WORLD_WIDTH: usize = 70;
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
pub const BODY_COLOR: Color = [0.0, 0.8, 0.0, 1.0];
pub const WALL_COLOR: Color = [0.8, 0.8, 0.8, 1.0];
pub const VOID_COLOR: Color = BACKGROUND_COLOR;
pub const APPLE_COLOR: Color = [1.0, 0.0, 0.0, 1.0];
pub const TEXT_COLOR: Color = [1.0, 1.0, 1.0, 1.0];

// Game constants

pub const STEPS_PER_SECOND: f64 = 15.0;
pub const UPDATE_DEALY: f64 = 1.0 / STEPS_PER_SECOND;

