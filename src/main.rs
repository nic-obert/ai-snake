mod consts;
mod map;
mod snake;
mod game_manager;
mod render;
mod brain;


use opengl_graphics::{GlGraphics, OpenGL};
use piston::{ButtonEvent, EventLoop};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;

use consts::*;
use piston_window::PistonWindow;


fn main() {

    let args = std::env::args().collect::<Vec<String>>();
    

    // Initialize graphics

    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new(WIN_TITLE, [WIN_WIDTH, WIN_HEIGHT])
        .exit_on_esc(true)
        .build()
        .expect("Could not create window");
    let mut gl = GlGraphics::new(opengl);

    let event_settings = EventSettings::new()
    // Lazy mode is disabled to allow updates to be independent of user input
        .lazy(false);
    let mut events = Events::new(event_settings);


    // Initialize game manager

    let mut game_manager = game_manager::GameManager::new(&mut window);

    match args.len() {
        1 => game_manager.initialize(),
        2 => {
            let gen_path = &args[1];
            game_manager.initialize_from_file(gen_path);
        },
        _ => panic!("Invalid number of arguments"),
    }


    // Game loop

    while let Some(event) = events.next(&mut window) {

        // Rendering
        if let Some(args) = event.render_args() {
            game_manager.draw(&args, &mut gl, &mut window, &event);
        }

        // Updating
        if let Some(args) = event.update_args() {
            game_manager.update(&args);
        }

        // Input
        if let Some(args) = event.button_args() {
            game_manager.handle_input(&args);
        }
    }

}

