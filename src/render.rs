use graphics::{Context, Transformed};
use graphics::types::Color;
use opengl_graphics::GlGraphics;
use piston::RenderArgs;
use piston_window::{Glyphs, PistonWindow};

use crate::consts::*;
use crate::map::Location;


pub trait Drawable {

    fn draw(&self, args: &RenderArgs, gl: &mut GlGraphics, window: &mut PistonWindow, event: &piston::Event);

}


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
            MAP_WIDTH + map_location.y as f64 * BLOCK_SIZE
        )
    }

}


pub fn render_block(color: Color, position: WindowCoordinates, context: &Context, gl: &mut GlGraphics) {

    let square = graphics::rectangle::square(
        position.x,
        position.y,
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

