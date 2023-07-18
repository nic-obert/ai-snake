use graphics::Context;
use opengl_graphics::GlGraphics;
use piston::RenderArgs;

use crate::map::Location;
use crate::utils::Drawable;
use crate::consts::*;



pub enum Direction {

    Up,
    Down,
    Left,
    Right,

}


pub struct Snake {

    length: usize,
    direction: Direction,
    pub bits: Vec<Location>

}


impl Snake {

    pub fn new(location: Location) -> Self {
        Snake {
            length: 1,
            direction: Direction::Up,
            bits: vec![location]
        }
    }


    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }


    pub fn add_bit(&mut self) {
        self.bits.push(*self.bits.last().unwrap());
        self.length += 1;
    }


    pub fn advance(&mut self) {
        // Move the body of the snake first
        for i in (1..self.length).rev() {
            self.bits[i] = self.bits[i - 1];
        }

        // Lastly, move the head
        let head = self.bits.first_mut().unwrap();
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
    }

}


impl Drawable for Snake {

    fn draw(&self, args: &RenderArgs, gl: &mut GlGraphics) {

        gl.draw(args.viewport(), |c: Context, gl: &mut GlGraphics| {

            // Draw the head first, as it is of a different color
            let head = self.bits.first().unwrap();
            let square = graphics::rectangle::square(
                (head.x as f64) * BLOCK_SIZE,
                (head.y as f64) * BLOCK_SIZE,
                BLOCK_SIZE
            );
            graphics::rectangle(HEAD_COLOR, square, c.transform, gl);

            // Draw the rest of the snake
            for bit in self.bits.iter().skip(1) {
                let square = graphics::rectangle::square(
                    (bit.x as f64) * BLOCK_SIZE,
                    (bit.y as f64) * BLOCK_SIZE,
                    BLOCK_SIZE
                );
                graphics::rectangle(BODY_COLOR, square, c.transform, gl);
            }

        })

    }

}

