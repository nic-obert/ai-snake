use piston_window::{Glyphs, PistonWindow};
use rand::Rng;

use crate::render::{render_text, WindowCoordinates, clear_screen, render_submap_matrix, render_borders};
use crate::snake::{Snake, Direction};
use crate::map::{Map, Location, Block};
use crate::apple::Apple;
use crate::render::Drawable;
use crate::consts::*;


enum GameStatus {

    GameOver,
    Running,
    Paused,

}


pub struct GameManager {

    game_status: GameStatus,
    pub snake: Snake,
    map: Map,
    apple: Option<Apple>,
    last_update: f64,
    font: Glyphs,

}


impl GameManager {

    pub fn new(font: Glyphs) -> Self {

        let snake_x = WORLD_WIDTH / 2;
        let snake_y = WORLD_HEIGHT / 2;
        let snake = Snake::new(Location::new(snake_x, snake_y));

        let map = Map::new();

        GameManager {
            game_status: GameStatus::Running,
            snake,
            map,
            apple: None,
            last_update: 0.0,
            font,
        }
    }


    pub fn update(&mut self, update_args: &piston::UpdateArgs) {

        // Limit update rate
        self.last_update += update_args.dt;
        if self.last_update >= UPDATE_DEALY {
            self.last_update = 0.0;
        } else {
            return;
        }

        // Don't update the game if it's not running
        if !matches!(self.game_status, GameStatus::Running) {
            return;
        }

        // Update the snake

        // Extract the submap around the snake's head
        self.snake.sight = Box::new(self.map.get_submap(self.snake.bits[0]));

        match self.snake.advance_and_update_map(&mut self.map) {
            Ok(..) => {},
            Err(..) => {
                // Snake collided with the wall
                self.game_over();
                return;
            }
        }

        // Check for other collisions

        // Snake collided with itself
        let head = self.snake.bits[0];
        for bit in self.snake.bits.iter().skip(1) {
            if *bit == head {
                self.game_over();
                return;
            }
        }

        // Snake collided with the apple
        if let Some(apple) = &self.apple {
            if self.snake.bits[0] == apple.location {
                self.snake.add_bit();
                self.map.free_block(apple.location);
                self.apple = None;
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
                self.map.set_apple_block(location);
                break;
            }
        }

    }


    pub fn handle_input(&mut self, args: &piston::ButtonArgs) {
        use piston::input::Key;

        if args.state == piston::input::ButtonState::Press {
            match args.button {
                piston::input::Button::Keyboard(key) => {
                    match key {
                        Key::W => self.snake.set_direction(Direction::Up),
                            
                        Key::S => self.snake.set_direction(Direction::Down),

                        Key::A => self.snake.set_direction(Direction::Left),

                        Key::D => self.snake.set_direction(Direction::Right),
                        
                        Key::Space => match self.game_status {
                            GameStatus::Running => self.pause(),
                            GameStatus::Paused => self.run(),
                            _ => {}
                        }
                        
                        // Unhandled keys
                        _ => {}
                    }
                },

                // Unhandled button types
                _ => {}
            }
            
        }
    }


    fn game_over(&mut self) {
        self.game_status = GameStatus::GameOver;
    }


    pub fn pause(&mut self) {
        self.game_status = GameStatus::Paused;
    }


    pub fn run(&mut self) {
        self.game_status = GameStatus::Running;
    }

}


impl Drawable for GameManager {

    fn draw(&mut self, args: &piston::RenderArgs, gl: &mut opengl_graphics::GlGraphics, window: &mut PistonWindow, event: &piston::Event) {

        // Clear the screen
        clear_screen(gl);

        // Draw the topbar
        render_text(
            &format!("Score: {}", self.snake.length),
            &mut self.font,
            WindowCoordinates::new(FONT_SIZE as f64, (TOPBAR_HEIGHT + FONT_SIZE as f64) / 2.0),
            window,
            event
        );

        // Draw the game elements

        self.map.draw(args, gl, window, event);
        self.snake.draw(args, gl, window, event);

        if let Some(apple) = &mut self.apple {
            apple.draw(args, gl, window, event);
        }

        match self.game_status {
            GameStatus::GameOver => {
                let text = "Game Over!";
                render_text(
                    text,
                    &mut self.font,
                    WindowCoordinates::new((WIN_WIDTH - (FONT_SIZE as f64 * text.len() as f64) / 2.0) / 2.0, (WIN_HEIGHT + FONT_SIZE as f64) / 2.0),
                    window,
                    event
                );
            },
            GameStatus::Paused => {
                let text = "Paused";
                render_text(
                    text,
                    &mut self.font,
                    WindowCoordinates::new((WIN_WIDTH - (FONT_SIZE as f64 * text.len() as f64) / 2.0) / 2.0, (WIN_HEIGHT + FONT_SIZE as f64) / 2.0),
                    window,
                    event
                );
            },
            _ => {}
            
        }

        // Draw the submap on the side panel

        let submap_start_x = SIDE_PANEL_START_X + 50.0;
        let submap_start_y = MAP_START_Y + FONT_SIZE as f64 + 50.0;
        
        render_text(
            "Input",
            &mut self.font,
            WindowCoordinates::new(submap_start_x, submap_start_y - FONT_SIZE as f64),
            window,
            event
        );

        render_borders(
            submap_start_x, 
            submap_start_y,
            submap_start_x + SUBMAP_BLOCK_SIZE * SIGHT_SIZE as f64,
            submap_start_y + SUBMAP_BLOCK_SIZE * SIGHT_SIZE as f64,
            SUBMAP_BORDER_THICKNESS,
            SUBMAP_BORDER_COLOR,
            window,
            event
        );

        render_submap_matrix(
            &self.snake.sight,
            WindowCoordinates::new(submap_start_x, submap_start_y),
            window,
            event
        );

        

    }

}

