
use opengl_graphics::TextureSettings;
use piston_window::{Glyphs, PistonWindow};

use crate::brain::Brain;
use crate::render::{render_text, WindowCoordinates, clear_screen};
use crate::snake::Snake;
use crate::map::{Map, Location};
use crate::render::Drawable;
use crate::{consts::*, font_path};


enum GameStatus {

    Running,
    Paused,

}


pub struct GameManager {

    game_status: GameStatus,
    map: Map,
    last_update: f64,
    font: Glyphs,
    generation_count: usize,
    snakes: Vec<Snake>,

}


/// Determines where to spawn a snake taking into account the other snakes
/// Returns the location where the snake should spawn
fn determine_snake_spawn_location(index: usize) -> Location {
    
    let x: usize = ((index as f64 % *GRID_SIZE) * *SECTION_SIZE_X + *SECTION_SIZE_X / 2.0) as usize;
    let y: usize = ((index as f64 / *GRID_SIZE).floor() * *SECTION_SIZE_Y + *SECTION_SIZE_Y / 2.0) as usize;

    Location::new(x, y)
}


impl GameManager {

    fn internal_initialize(&mut self, initialize_snakes: bool) {

        self.game_status = GameStatus::Running;

        self.map = Map::create_new();

        if initialize_snakes {

            let mut snakes = Vec::with_capacity(GENERATION_SIZE);

            for i in 0..GENERATION_SIZE {
                snakes.push(Snake::spawn_new(
                    determine_snake_spawn_location(i),
                    &mut self.map
                ));
            }

        }

        for _ in 0..MAX_APPLES {
            self.map.spawn_apple();
        }

    }


    /// Initialize the game manager and the game
    pub fn initialize(&mut self) {
        self.internal_initialize(true);
    }


    /// Save the current generation to a file
    fn save_generation(&self) {
        use std::fs::File;
        use std::io::Write;

        let file_name = format!("Gen_{}_{}.gen.json", self.generation_count, chrono::Local::now().format("%Y-%m-%d_%H-%M-%S"));

        let mut file = File::create(&file_name).expect(
            "Failed to create generation file"
        );

        let brains: Vec<&Brain> = self.snakes.iter().map(|x| &x.brain).collect();

        let json = serde_json::to_string_pretty(&brains).expect(
            "Failed to serialize the brains"
        );

        file.write_all(json.as_bytes()).expect(
            "Failed to write to generation file"
        );

        println!("Generation {} saved to file \"{}\"", self.generation_count, file_name);
    }

    
    /// Reset the game and the generation
    fn reset_all(&mut self) {
        self.reset_game();
        self.generation_count = 1;
        self.snakes.clear();
        
        for i in 0..GENERATION_SIZE {
            self.snakes.push(Snake::spawn_new(
                determine_snake_spawn_location(i),
                &mut self.map
            ));
        }
    }


    /// Selects the best snakes among the current generation
    /// Keeps the longest snakes and discards the short ones
    /// Empties the snakes vector and returns the selected snakes
    fn select_best_snakes(&mut self) -> Vec<Snake> {

        // Discard the short snakes
        self.snakes.retain(|x| x.length() > INITIAL_SNAKE_LENGTH);

        // Sort the snakes by length in descending order
        self.snakes.sort_by(|a, b| b.length().cmp(&a.length()));

        // Keep only the longest snakes
        self.snakes.truncate(GENERATION_CARRYOVER);

        self.snakes.drain(..).collect()
    }


    /// Pass to the next generation and reset the game
    fn next_generation(&mut self) {

        // Increment the generation counter
        self.generation_count += 1;

        println!("\nGeneration: {}\n", self.generation_count);

        // Select the snakes to breed and repopulate the generation
        let mut best_snakes = self.select_best_snakes();

        println!("Good snakes in this generation: {}", best_snakes.len());
        for (i, snake) in best_snakes.iter().enumerate() {
            println!("{}. Snake length: {}", i+1, snake.length());
        }
        println!();

        if best_snakes.is_empty() {
            // If there are no good snakes, repopulate the generation with new random snakes
            for _ in 0..GENERATION_SIZE {
                self.snakes.push(
                    Snake::spawn_new(
                        determine_snake_spawn_location(self.snakes.len()),
                        &mut self.map
                    ));
            }

        } else {
            // If there are good snakes, repopulate the generation with offsprings of the best snakes
            while self.snakes.len() < GENERATION_SIZE - best_snakes.len() {
                for snake in best_snakes.iter() {
                    self.snakes.push(snake.spawn_offspring(
                        determine_snake_spawn_location(self.snakes.len()),
                        &mut self.map
                    ));
                }
            }

            // Add the best snakes from the previous generation to the new generation
            self.snakes.append(&mut best_snakes);
        }

        self.reset_game();

    }


    /// Reset game parameters and the map
    fn reset_game(&mut self) {
        self.last_update = 0.0;
        self.map = Map::create_new();

        // Respawn the apples
        for _ in 0..MAX_APPLES {
            self.map.spawn_apple();
        }
    }


    pub fn initialize_from_file(&mut self, path: &str) {

        // Load the json file
        let json = std::fs::read_to_string(path).expect(
            format!("Failed to read the file: {:?}", path).as_str()
        );

        // Deserialize the json file
        let brains: Vec<Brain> = serde_json::from_str(&json).expect(
            format!("Failed to deserialize the json file: {:?}", path).as_str()
        );

        // Initialize the game manager
        self.internal_initialize(false);

        // Create the snakes from the brains
        for brain in brains {
            self.snakes.push(Snake::spawn_with_brain(
                brain,
                determine_snake_spawn_location(self.snakes.len()),
                &mut self.map
            ));
        }

        println!("Loaded generation from file: {:?}", path)

    }


    pub fn new(window: &mut PistonWindow) -> Self {

        let font = include_bytes!(font_path!());

        let glyphs = Glyphs::from_bytes(
            font,
            window.create_texture_context(),
            TextureSettings::new(),
        ).unwrap();

        Self {
            game_status: GameStatus::Running,
            snakes: Vec::new(),
            map: Map::empty_new(),
            last_update: 0.0,
            font: glyphs,
            generation_count: 1,
        }
    }


    /// Limits the update rate to a fixed rate
    /// Takes into account the game state to determine if the game should be updated
    /// Returns true if the game should be updated
    /// Returns false if the game should not be updated
    fn tick(&mut self, update_args: &piston::UpdateArgs) -> bool {

        // Don't update the game if it's not running
        if !matches!(self.game_status, GameStatus::Running) {
            return false;
        }

        self.last_update += update_args.dt;
        if self.last_update >= UPDATE_DEALY {
            self.last_update = 0.0;
            true
        } else {
            false
        }
    }


    /// Update the game 
    pub fn update(&mut self, update_args: &piston::UpdateArgs) {

        // Limit the update rate
        if !self.tick(update_args) {
            return;
        }

        // Update the game elements
        let mut population_count: usize = 0;
        for snake in &mut self.snakes {

            if snake.alive {
                population_count += 1;
                snake.act(&mut self.map);
            }
            
        }

        // Check if the game is over, if so, pass to the next generation
        if population_count == 0 {
            std::thread::sleep(NEXT_GENERATION_DELAY);
            self.next_generation();
        }

    }


    /// Handle user input
    pub fn handle_input(&mut self, args: &piston::ButtonArgs) {
        use piston::input::Key;

        if args.state == piston::input::ButtonState::Press {
            match args.button {
                piston::input::Button::Keyboard(key) => {
                    match key {
                        
                        Key::Space => match self.game_status {
                            GameStatus::Running => self.pause(),
                            GameStatus::Paused => self.unpause(),
                        }

                        Key::Return => self.next_generation(),

                        Key::R => self.reset_all(),

                        Key::S => self.save_generation(),
                        
                        // Unhandled keys
                        _ => {}
                    }
                },

                // Unhandled button types
                _ => {}
            }
            
        }
    }


    /// Pause the game
    fn pause(&mut self) {
        self.game_status = GameStatus::Paused;
    }


    /// Unpause the game
    fn unpause(&mut self) {
        self.game_status = GameStatus::Running;
    }


    /// Draw the game on the screen
    pub fn draw(&mut self, args: &piston::RenderArgs, gl: &mut opengl_graphics::GlGraphics, window: &mut PistonWindow, event: &piston::Event) {

        // Clear the screen
        clear_screen(gl);

        // Draw the topbar
        render_text(
            &format!("Generation: {}", self.generation_count),
            &mut self.font,
            WindowCoordinates::new(FONT_SIZE as f64, (TOPBAR_HEIGHT + FONT_SIZE as f64) / 2.0),
            window,
            event
        );

        // Draw the game elements

        self.map.draw(args, gl, window, event);

        match self.game_status {
            GameStatus::Paused => {
                let text = "Paused";
                render_text(
                    text,
                    &mut self.font,
                    WindowCoordinates::new(
                        (WIN_WIDTH - (FONT_SIZE as f64 * text.len() as f64) / 2.0) / 2.0,
                        (WIN_HEIGHT + FONT_SIZE as f64) / 2.0
                    ),
                    window,
                    event
                );
            },
            _ => {}
            
        }        

    }

}

