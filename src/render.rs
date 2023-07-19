use graphics::{Context, Transformed};
use graphics::types::Color;
use opengl_graphics::GlGraphics;
use piston_window::{Glyphs, PistonWindow};

use crate::consts::*;
use crate::map::Location;


pub struct WindowCoordinates {
    pub x: f64,
    pub y: f64,
}


impl WindowCoordinates {

    pub fn new(x: f64, y: f64) -> Self {
        WindowCoordinates {
            x,
            y
        }
    }


    pub fn from_map_location(map_location: Location) -> WindowCoordinates {
        WindowCoordinates::new(
            map_location.x as f64 * BLOCK_SIZE,
            MAP_START_Y + map_location.y as f64 * BLOCK_SIZE
        )
    }

}


pub fn render_block(color: Color, location: Location, context: &Context, gl: &mut GlGraphics) {

    let win_coordinates = WindowCoordinates::from_map_location(location);

    let square = graphics::rectangle::square(
        win_coordinates.x,
        win_coordinates.y,
        BLOCK_SIZE
    );

    graphics::rectangle(color, square, context.transform, gl);
}


pub fn render_text(text: &str, font: &mut Glyphs, coordinates: WindowCoordinates, window: &mut PistonWindow, event: &piston::Event) {

    window.draw_2d(event, |context, graphics, device| {

        let transform = context.transform.trans(
            coordinates.x,
            coordinates.y
        );

        graphics::text(TEXT_COLOR, FONT_SIZE, text, font, transform, graphics)
            .unwrap();

        font.factory.encoder.flush(device);

    });
}


pub fn clear_screen(gl: &mut GlGraphics) {
    graphics::clear(BACKGROUND_COLOR, gl);
}

