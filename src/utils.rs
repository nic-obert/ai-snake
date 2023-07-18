use opengl_graphics::GlGraphics;
use piston::RenderArgs;


pub trait Drawable {

    fn draw(&self, args: &RenderArgs, gl: &mut GlGraphics);

}

