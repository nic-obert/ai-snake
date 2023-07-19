use graphics::Context;
use opengl_graphics::GlGraphics;
use piston::RenderArgs;
use piston_window::PistonWindow;

use crate::brain::Brain;
use crate::map::{Location, Map, Block, SubmapMatrix};
use crate::render::{render_block, Drawable, WindowCoordinates};
use crate::consts::*;


pub enum Direction {

    Up,
    Down,
    Left,
    Right,

}


pub struct Snake {

    pub length: usize,
    direction: Direction,
    pub bits: Vec<Location>,
    brain: Brain,
    pub sight: Box<SubmapMatrix>,

}


impl Snake {

    pub fn new(location: Location) -> Self {
        Snake {
            length: 1,
            direction: Direction::Up,
            bits: vec![location],
            brain: Brain::new(),
            sight: Box::new([[Block::Void; SIGHT_SIZE]; SIGHT_SIZE]),
        }
    }


    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }


    pub fn add_bit(&mut self) {
        self.bits.push(*self.bits.last().unwrap());
        self.length += 1;
    }


    pub fn advance_and_update_map(&mut self, map: &mut Map) -> Result<(), ()> {

        // Remove the last bit of the snake from the map
        let last_bit = self.bits.last().unwrap();
        map.free_block(*last_bit);

        // Move the body of the snake first
        for i in (1..self.length).rev() {
            self.bits[i] = self.bits[i - 1];
        }

        // Lastly, move the head
        let head = &mut self.bits[0];
        match self.direction {
            Direction::Up => {
                head.y -= 1;
            },
            Direction::Down => {
                head.y += 1;
            },
            Direction::Left => {
                head.x -= 1;
            },
            Direction::Right => {
                head.x += 1;
            }
        }

        // Check if the snake collided with the wall
        if map.get(*head) == Block::Wall {
            return Err(());
        }

        // Occupy the new location of the head
        map.set_snake_block(*head);

        Ok(())
    }

}


impl Drawable for Snake {

    fn draw(&mut self, args: &RenderArgs, gl: &mut GlGraphics, _window: &mut PistonWindow, _event: &piston::Event) {

        gl.draw(args.viewport(), |context: Context, gl: &mut GlGraphics| {

            // Draw the head first, as it is of a different color
            let head = self.bits[0];
            render_block(
                HEAD_COLOR,
                WindowCoordinates::from_map_location(head),
                &context,
                gl
            );

            // Draw the rest of the snake
            for bit in self.bits.iter().skip(1) {
                render_block(
                    BODY_COLOR,
                    WindowCoordinates::from_map_location(*bit),
                    &context,
                    gl
                );
            }

        })

    }

}

