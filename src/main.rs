extern crate piston_window;

use piston_window::*;

struct Game {
    rotation: f64,
}

impl Game {
    fn new() -> Game {
        Game { rotation: 0.0 }
    }

    fn on_update(&mut self, update: UpdateArgs) {
        self.rotation += 0.7 * update.dt;
    }

    fn on_draw(&mut self, render: RenderArgs, e: PistonWindow) {
        e.draw_2d(|c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            let center = c.transform.trans((render.width / 2) as f64, (render.height / 2) as f64);
            let square = rectangle::square(0.0, 0.0, 100.0);
            let gray = [0.7, 0.7, 0.7, 1.0];
            rectangle(gray, square, center.rot_rad(self.rotation).trans(-50.0, -50.0), g);
        });
    }
}

fn main() {
    let window: PistonWindow = WindowSettings::new("mercury", [600, 600])
        .exit_on_esc(true).build().unwrap();

    let mut game = Game::new();

    for e in window {

        match e.event {
            Some(Event::Update(update)) => game.on_update(update),
            Some(Event::Render(render)) => game.on_draw(render, e),
            _ => {}
        }
    }
}
