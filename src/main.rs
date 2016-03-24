extern crate piston_window;

use piston_window::*;

fn main() {
    let window: PistonWindow = WindowSettings::new("mercury", [600, 600])
        .exit_on_esc(true).build().unwrap();

    for _e in window {

    }
}
