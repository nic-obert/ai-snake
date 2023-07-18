mod consts;
mod map;
mod snake;
mod utils;
mod game_manager;
mod apple;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{ButtonEvent, EventLoop};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;

use consts::*;
use utils::Drawable;


fn main() {
    
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Snake", [WIN_WIDTH, WIN_HEIGHT])
        .exit_on_esc(true);
    let mut window: GlutinWindow = settings.build().expect("Could not create window");
    let mut gl = GlGraphics::new(opengl);
   
    let mut game_manager = game_manager::GameManager::new();

    let event_settings = EventSettings::new()
        .ups(UPS);
    let mut events = Events::new(event_settings);
    while let Some(e) = events.next(&mut window) {

        // Rendering
        if let Some(args) = e.render_args() {
            game_manager.draw(&args, &mut gl);
        }

        // Updating
        if let Some(args) = e.update_args() {
            game_manager.update(&args);
        }

        // Input
        if let Some(args) = e.button_args() {
            game_manager.handle_input(&args);
        }
    }


}
