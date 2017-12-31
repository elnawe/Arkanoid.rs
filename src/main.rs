extern crate piston;

mod core;

use core::window;
use piston::event_loop::*;
use piston::input::*;

fn main() {
    let mut window = window::new(
        String::from("Arkanoid.rs"),
        [640, 480]
    );
    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window.frame) {
        if let Some(r) = e.render_args() {
            window.render(&r);
        }
    }
}
