use opengl_graphics::GlGraphics;
use piston::RenderArgs;
use piston_window::PistonWindow;


pub trait Drawable {

    fn draw(&mut self, args: &RenderArgs, gl: &mut GlGraphics, window: &mut PistonWindow, event: &piston::Event);

}

