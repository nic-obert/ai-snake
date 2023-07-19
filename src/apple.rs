use crate::utils::Drawable;
use crate::map::Location;
use crate::consts::*;


pub struct Apple {

    pub location: Location,

}


impl Apple {

    pub fn new(location: Location) -> Self {
        Apple {
            location
        }
    }

}


impl Drawable for Apple {

    fn draw(&self, args: &piston::RenderArgs, gl: &mut opengl_graphics::GlGraphics) {
        use graphics::*;

        let square = rectangle::square(
            self.location.x as f64 * BLOCK_SIZE,
            self.location.y as f64 * BLOCK_SIZE,
            BLOCK_SIZE
        );

        gl.draw(args.viewport(), |c, gl| {
            rectangle(APPLE_COLOR, square, c.transform, gl);
        });
    }

}

