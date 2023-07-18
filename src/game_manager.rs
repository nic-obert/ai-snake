use rand::Rng;

use crate::snake::{Snake, Direction};
use crate::map::{Map, Location, Block};
use crate::apple::Apple;
use crate::utils::Drawable;
use crate::consts::*;


pub struct GameManager {

    game_over: bool,
    pub snake: Snake,
    map: Map,
    apple: Option<Apple>,

}


impl GameManager {

    pub fn new() -> Self {

        let snake_x = WORLD_WIDTH / 2;
        let snake_y = WORLD_HEIGHT / 2;
        let snake = Snake::new(Location::new(snake_x, snake_y));

        let map = Map::new(WORLD_WIDTH, WORLD_HEIGHT, BLOCK_SIZE);

        GameManager {
            game_over: false,
            snake,
            map,
            apple: None,
        }
    }


    pub fn update(&mut self, update_args: &piston::UpdateArgs) {

        if self.game_over {
            return;
            // TODO: add a game over screen
        }

        self.snake.advance();

        // Check for collisions

        // Snake collided with itself
        let head = self.snake.bits.first().unwrap();
        for bit in self.snake.bits.iter().skip(1) {
            if bit == head {
                self.game_over = true;
                return;
            }
        }

        // Snake collided with the apple
        if let Some(apple) = &self.apple {
            if self.snake.bits.first().unwrap() == &apple.location {
                self.snake.add_bit();
                self.apple = None;
            }
        }

        // Snake collided with the wall
        if let Some(head) = self.snake.bits.first() {
            match self.map.blocks[head.y][head.x] {
                Block::Wall => {
                    self.game_over = true;
                    return;
                },
                _ => {}
            }
        }

        // Spawn an apple if there isn't one
        if self.apple.is_none() {
            let mut rng = rand::thread_rng();

            loop {

                let x = rng.gen_range(0..WORLD_WIDTH);
                let y = rng.gen_range(0..WORLD_HEIGHT);
                let location = Location::new(x, y);

                // Check if the apple is in a valid location (not in a wall or the snake)
                if self.map.blocks[y][x] == Block::Wall {
                    continue;
                }
                for bit in self.snake.bits.iter() {
                    if bit == &location {
                        continue;
                    }
                }

                self.apple = Some(Apple::new(location));
                break;
            }
        }

    }


    pub fn handle_input(&mut self, args: &piston::ButtonArgs) {
        if args.state == piston::input::ButtonState::Press {
            match args.button {
                piston::input::Button::Keyboard(key) => {
                    match key {
                        piston::input::Key::W => {
                            self.snake.set_direction(Direction::Up)
                        },
                        piston::input::Key::S => {
                            self.snake.set_direction(Direction::Down)
                        },
                        piston::input::Key::A => {
                            self.snake.set_direction(Direction::Left)
                        },
                        piston::input::Key::D => {
                            self.snake.set_direction(Direction::Right)
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        }
    }

}


impl Drawable for GameManager {

    fn draw(&self, args: &piston::RenderArgs, gl: &mut opengl_graphics::GlGraphics) {
        self.map.draw(args, gl);
        self.snake.draw(args, gl);

        if let Some(apple) = &self.apple {
            apple.draw(args, gl);
        }
    }

}

