use graphics::{types::Color, Context};
use opengl_graphics::GlGraphics;
use piston::RenderArgs;

use crate::render::{render_block, Drawable, WindowCoordinates};
use crate::consts::*;


pub type SubmapMatrix = [[Block; SIGHT_SIZE]; SIGHT_SIZE];


#[derive(Clone, Copy, PartialEq)]
pub struct Location {

    pub x: usize,
    pub y: usize,

}


impl Location {

    pub fn new(x: usize, y: usize) -> Self {
        Location {
            x,
            y
        }
    }

}


#[derive(Clone, Copy, PartialEq)]
pub enum Block {
    Void,
    Wall,
    Snake,
    Apple,
}


impl Block {

    pub fn color(&self) -> Color {
        match self {
            Block::Void => VOID_COLOR,
            Block::Wall => WALL_COLOR,
            Block::Snake => BODY_COLOR,
            Block::Apple => APPLE_COLOR,
        }
    }

}


pub struct Map {

    pub blocks: Vec<Vec<Block>>

}


impl Map {

    pub fn new() -> Self {

        let mut blocks = Vec::with_capacity(WORLD_HEIGHT as usize);
        
        // Construct the walls when creating the map

        blocks.push(vec![Block::Wall; WORLD_WIDTH as usize]);

        for _ in 1..WORLD_HEIGHT-1 {
            let mut row = Vec::with_capacity(WORLD_WIDTH as usize);
            row.push(Block::Wall);
            for _ in 1..WORLD_WIDTH-1 {
                row.push(Block::Void);
            }
            row.push(Block::Wall);
            blocks.push(row);
        }

        blocks.push(vec![Block::Wall; WORLD_WIDTH as usize]);

        Map {
            blocks
        }

    }


    pub fn get(&self, location: Location) -> Block {
        self.blocks[location.y][location.x]
    }


    /// Sets the given location as blocked by the snake
    pub fn set_snake_block(&mut self, location: Location) {
        self.blocks[location.y][location.x] = Block::Snake;
    }


    /// Frees the given location from the snake
    pub fn free_block(&mut self, location: Location) {
        self.blocks[location.y][location.x] = Block::Void;
    }


    /// Sets the given location as occupied by the apple
    pub fn set_apple_block(&mut self, location: Location) {
        self.blocks[location.y][location.x] = Block::Apple;
    }


    /// Returns the submap centered around the given location
    pub fn get_submap(&self, center: Location) -> SubmapMatrix {
        // Calculate the top left corner of the submap
        let top_left_x = center.x as i64 - SIGHT_RADIUS as i64;
        let top_left_y = center.y as i64 - SIGHT_RADIUS as i64;
        
        // Initialize the submap
        let mut submap = [[Block::Void; SIGHT_SIZE]; SIGHT_SIZE];

        // Copy the blocks from the world map to the submap
        for (y, submap_row) in submap.iter_mut().enumerate() {

            // Check if the coordinates are out of bounds
            let y = top_left_y + y as i64;
            if y < 0 || y >= WORLD_HEIGHT as i64 {
                continue;
            }

            for (x, block) in submap_row.iter_mut().enumerate() {

                // Check if the coordinates are out of bounds
                let x = top_left_x + x as i64;
                if x < 0 || x >= WORLD_WIDTH as i64 {
                    continue;
                }

                *block = self.blocks[y as usize][x as usize];
  
            }
        }

        submap
    }

}


impl Drawable for Map {

    fn draw(&mut self, args: &RenderArgs, gl: &mut GlGraphics, _window: &mut piston_window::PistonWindow, _event: &piston::Event) {

        gl.draw(args.viewport(), |context: Context, gl: &mut GlGraphics| {

            // Draw the world map block by block
            for (y, row) in self.blocks.iter().enumerate() {
                for (x, block) in row.iter().enumerate() {
                    render_block(
                        block.color(),
                        WindowCoordinates::from_map_location(Location::new(x, y)),
                        &context,
                        gl
                    )
                }
            }

        })

    }

}

