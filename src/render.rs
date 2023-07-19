use graphics::rectangle::square;
use graphics::{Context, Transformed};
use graphics::types::Color;
use opengl_graphics::GlGraphics;
use piston::RenderArgs;
use piston_window::{Glyphs, PistonWindow};

use crate::consts::*;
use crate::map::{Location, SubmapMatrix};


pub trait Drawable {

    fn draw(&mut self, args: &RenderArgs, gl: &mut GlGraphics, window: &mut PistonWindow, event: &piston::Event);

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
            MAP_START_Y + map_location.y as f64 * BLOCK_SIZE
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


pub fn render_submap_matrix(matrix: &SubmapMatrix, coordinates: WindowCoordinates, window: &mut PistonWindow, event: &piston::Event) {

    window.draw_2d(event, |context, graphics, _device| {

        for (y, row) in matrix.iter().enumerate() {
            for (x, block) in row.iter().enumerate() {

                let square = square(
                    coordinates.x + x as f64 * SUBMAP_BLOCK_SIZE,
                    coordinates.y + y as f64 * SUBMAP_BLOCK_SIZE,
                    SUBMAP_BLOCK_SIZE
                );

                graphics::rectangle(block.color(), square, context.transform, graphics);

            }
        }

        
    });

}


pub fn render_borders(x1: f64, y1: f64, x2: f64, y2: f64, thickness: f64, color: Color, window: &mut PistonWindow, event: &piston::Event) {
    
    window.draw_2d(event, |context, graphics, _device| {

        let square = graphics::rectangle::rectangle_by_corners(
            x1 - thickness,
            y1 - thickness,
            x2 + thickness,
            y2 + thickness
        );
    
        graphics::rectangle(color, square, context.transform, graphics);

    });

}

