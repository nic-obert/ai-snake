use graphics::{types::Color, Context};
use opengl_graphics::GlGraphics;
use piston::RenderArgs;

use crate::render::{render_block, Drawable, WindowCoordinates};
use crate::consts::*;
use crate::snake::Direction;


pub type SubmapMatrix = [[Block; SIGHT_SIZE]; SIGHT_SIZE];


#[derive(Clone, Copy, PartialEq, Debug)]
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


    /// Returns the new location after moving in the given direction
    pub fn trans(&self, amount: i64, direction: Direction) -> Self {
        match direction {
            Direction::Up => Location::new(self.x, (self.y as i64 - amount) as usize),
            Direction::Down => Location::new(self.x, (self.y as i64 + amount) as usize),
            Direction::Left => Location::new((self.x as i64 - amount) as usize, self.y),
            Direction::Right => Location::new((self.x as i64 + amount) as usize, self.y),
        }
    }

}


#[derive(Clone, Copy, PartialEq)]
pub enum Block {
    Void,
    Wall,
    SnakeTail,
    SnakeHead,
    Apple,
}


impl Block {

    pub fn color(&self) -> Color {
        match self {
            Block::Void => VOID_COLOR,
            Block::Wall => WALL_COLOR,
            Block::SnakeTail => TAIL_COLOR,
            Block::Apple => APPLE_COLOR,
            Block::SnakeHead => HEAD_COLOR,
        }
    }

}


pub struct Map {

    pub blocks: Vec<Vec<Block>>

}


impl Map {

    // Spawn an apple in a random valid location
    pub fn spawn_apple(&mut self) {

        let mut new_location = Location::new(0, 0);

        loop {

            new_location.x = rand::random::<usize>() % WORLD_WIDTH;
            new_location.y = rand::random::<usize>() % WORLD_HEIGHT;

            if self.get(new_location) == Block::Void {
                break;
            }
        }

        self.set_apple_block(new_location);

    }


    /// Create a new empty map, used as a placeholder
    pub fn empty_new() -> Self {
        Self {
            blocks: Vec::new()
        }
    }


    /// Create a new complete map with the walls
    pub fn create_new() -> Self {

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


    /// Returns the block at the given location
    pub fn get(&self, location: Location) -> Block {
        self.blocks[location.y][location.x]
    }


    /// Sets the given location as blocked by the snake head
    pub fn set_head_block(&mut self, location: Location) {
        self.blocks[location.y][location.x] = Block::SnakeHead;
    }


    /// Sets the given location as blocked by the snake tail
    pub fn set_tail_block(&mut self, location: Location) {
        self.blocks[location.y][location.x] = Block::SnakeTail;
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

    fn draw(&self, args: &RenderArgs, gl: &mut GlGraphics, _window: &mut piston_window::PistonWindow, _event: &piston::Event) {

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

