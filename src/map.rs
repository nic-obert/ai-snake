use graphics::{types::Color, Context};
use opengl_graphics::GlGraphics;
use piston::RenderArgs;

use crate::utils::Drawable;
use crate::consts::*;


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



#[derive(Clone, PartialEq)]
pub enum Block {
    Void,
    Wall,
}


impl Block {

    pub fn color(&self) -> Color {
        match self {
            Block::Void => VOID_COLOR,
            Block::Wall => WALL_COLOR,
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

}


impl Drawable for Map {

    fn draw(&self, args: &RenderArgs, gl: &mut GlGraphics) {

        gl.draw(args.viewport(), |c: Context, gl: &mut GlGraphics| {

            // Draw the world map block by block
            for (y, row) in self.blocks.iter().enumerate() {
                for (x, block) in row.iter().enumerate() {
                    let square = graphics::rectangle::square(
                        (x as f64) * BLOCK_SIZE,
                        (y as f64) * BLOCK_SIZE,
                        BLOCK_SIZE
                    );
                    graphics::rectangle(block.color(), square, c.transform, gl);
                }
            }

        })

    }

}

