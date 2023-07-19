mod consts;
mod map;
mod snake;
mod game_manager;
mod apple;
mod render;
mod brain;


use opengl_graphics::{GlGraphics, OpenGL, TextureSettings};
use piston::{ButtonEvent, EventLoop};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;

use consts::*;
use piston_window::{Glyphs, PistonWindow};
use render::Drawable;


fn main() {
    
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new(WIN_TITLE, [WIN_WIDTH, WIN_HEIGHT])
        .exit_on_esc(true)
        .build()
        .expect("Could not create window");
    let mut gl = GlGraphics::new(opengl);

    let font = include_bytes!(font_path!());

    let glyphs = Glyphs::from_bytes(
        font,
        window.create_texture_context(),
        TextureSettings::new(),
    )
    .unwrap();
   
    let mut game_manager = game_manager::GameManager::new(glyphs);

    let event_settings = EventSettings::new()
        .lazy(false);
    let mut events = Events::new(event_settings);

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

