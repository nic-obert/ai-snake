use graphics::Context;
use opengl_graphics::GlGraphics;
use piston::RenderArgs;
use piston_window::PistonWindow;

use crate::brain::Brain;
use crate::map::{Location, Map, Block, SubmapMatrix};
use crate::render::{render_block, Drawable, WindowCoordinates};
use crate::consts::*;


#[derive(Clone, Copy, PartialEq)]
pub enum Direction {

    Up,
    Down,
    Left,
    Right,

}


impl Direction {

    pub fn random() -> Self {
        match rand::random::<usize>() % 4 {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Right,
            _ => panic!("Invalid random number"),
        }
    }


    pub fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

}


pub type SnakeBody = Vec<Location>;


#[derive(Clone)]
pub struct Snake {

    direction: Direction,
    pub bits: SnakeBody,
    pub brain: Brain,
    pub alive: bool,

}


impl Snake {

    pub fn length(&self) -> usize {
        self.bits.len()
    }


    /// Updates the snake and makes it act in the game
    /// Updates the map accordingly
    pub fn act(&mut self, map: &mut Map) {

        let sight = map.get_submap(self.bits[0]);
    
        self.choose_direction(&sight);

        self.advance_and_update_map(map);
    }


    pub fn spawn_with_brain(brain: Brain, location: Location, map: &mut Map) -> Self {

        let direction = Direction::random();

        Snake {
            direction,
            bits: Snake::spawn_from_head(location, direction, map),
            brain,
            alive: true,
        }
    }


    /// Spawns a new snake with the same brain as the parent at the given location
    pub fn spawn_offspring(&self, location: Location, map: &mut Map) -> Self {

        let direction = Direction::random();

        let mut offspring = Snake {
            direction,
            bits: Snake::spawn_from_head(location, direction, map),
            brain: self.brain.clone(),
            alive: true,
        };

        offspring.brain.mutate();

        offspring
    }


    fn spawn_from_head(head: Location, direction: Direction, map: &mut Map) -> SnakeBody {

        let mut bits = Vec::with_capacity(INITIAL_SNAKE_LENGTH);

        bits.push(head);
        map.set_head_block(head);

        for i in 1..INITIAL_SNAKE_LENGTH {
            let bit_location = head.trans(i as i64, direction.opposite());
            map.set_tail_block(bit_location);
            bits.push(bit_location);
        }

        bits
    }


    /// Spawns a new snake on the map
    pub fn spawn_new(head_location: Location, map: &mut Map) -> Self {

        let direction = Direction::random();
        
        Snake {
            direction,
            bits: Snake::spawn_from_head(head_location, direction, map),
            brain: Brain::new(),
            alive: true,
        }
    }


    /// Sets the direction of the snake
    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }


    /// Adds a new bit to the snake
    pub fn add_bit(&mut self) {
        self.bits.push(*self.bits.last().unwrap());
    }


    /// Kills the snake and frees the blocks it occupied
    pub fn die(&mut self, map: &mut Map) {

        self.alive = false;

        // Skip the head, since it wasn't updaten on the map
        for bit in self.bits.iter() {
            map.free_block(*bit);
        }
    }


    /// Move the snake forward and update the map accordingly
    /// Checks for collisions and deaths
    pub fn advance_and_update_map(&mut self, map: &mut Map) {

        // Calculate new position of the new head
        let mut new_head = self.bits[0];
        match self.direction {
            Direction::Up => {
                new_head.y -= 1;
            },
            Direction::Down => {
                new_head.y += 1;
            },
            Direction::Left => {
                new_head.x -= 1;
            },
            Direction::Right => {
                new_head.x += 1;
            }
        }

        //println!("Snake moved to location {:?}", head);

        // Check if the snake collided with something and update the map accordingly
        match map.get(new_head) {

            Block::Wall |
            Block::SnakeTail |
            Block::SnakeHead => {
                // The snake collided with the wall or itself
                self.die(map);
            },

            Block::Apple => {
                // Occupy the new location of the head if the snake didn't die
                map.set_head_block(new_head);

                // Increase the length of the snake
                self.add_bit();

                // Don't remove the last bit of the snake from the map since it grew
            },

            Block::Void => {
                // Occupy the new location of the head if the snake didn't die
                map.set_head_block(new_head);

                // Remove the last bit of the snake from the map
                map.free_block(*self.bits.last().unwrap());
            }

        };

        // If the snake died, don't move it
        if !self.alive {
            return;
        }

        // Move the body of the snake first
        for i in (1..self.length()).rev() {
            self.bits[i] = self.bits[i - 1];
            // Update the map accordingly
            map.set_tail_block(self.bits[i]);
        }

        // Move the head of the snake
        self.bits[0] = new_head;

    }


    /// Calculates a new direction for the snake based on the sight input
    pub fn choose_direction(&mut self, sight_input: &SubmapMatrix) {
        let direction = self.brain.think(sight_input);
        self.set_direction(direction);
    }

}


impl Drawable for Snake {

    fn draw(&self, args: &RenderArgs, gl: &mut GlGraphics, _window: &mut PistonWindow, _event: &piston::Event) {

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
                    TAIL_COLOR,
                    WindowCoordinates::from_map_location(*bit),
                    &context,
                    gl
                );
            }

        })

    }

}

