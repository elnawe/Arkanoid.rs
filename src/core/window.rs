extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use self::glutin_window::GlutinWindow;
use self::opengl_graphics::{GlGraphics, OpenGL};
use self::piston::input::*;
use self::piston::window::WindowSettings;

pub fn new(title: String, resolution: [u32; 2]) -> Window {
    let opengl = OpenGL::V3_2;
    
    let frame = WindowSettings::new(
                        title, 
                        resolution
                    )
                    .opengl(opengl)
                    .exit_on_esc(true)
                    .build()
                    .unwrap();

    Window {
        frame: frame,
        gl: GlGraphics::new(opengl)
    }
}

pub struct Window {
    pub frame: GlutinWindow,
    gl: GlGraphics
}

impl Window {
    pub fn render(&mut self, args: &RenderArgs) {
        use self::graphics::clear;

        self.gl.draw(args.viewport(), |_, gl| {
            clear([0.0, 1.0, 0.0, 1.0], gl);
        });
    }
}