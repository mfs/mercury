extern crate piston_window;

use piston_window::*;

struct Game {
    rotation: f64,
    x: f64, y: f64,
    up_d: bool, down_d: bool, left_d: bool, right_d: bool,
}

impl Game {
    fn new() -> Game {
        Game {
            rotation: 0.0, x: 0.0, y: 0.0,
            up_d: false, down_d: false, left_d: false, right_d: false
        }
    }

    fn on_update(&mut self, update: UpdateArgs) {
        self.rotation += 0.7 * update.dt;

        if self.up_d {
           self.y += (-50.0) * update.dt;
        }
        if self.down_d {
            self.y += (50.0) * update.dt;
        }
        if self.left_d {
            self.x += (-50.0) * update.dt;
        }
        if self.right_d {
            self.x += (50.0) * update.dt;
        }
    }

    fn on_draw(&mut self, render: RenderArgs, e: PistonWindow) {
        e.draw_2d(|c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            let center = c.transform.trans((render.width / 2) as f64, (render.height / 2) as f64);
            let square = rectangle::square(0.0, 0.0, 100.0);
            let gray = [0.7, 0.7, 0.7, 1.0];
            rectangle(gray, square, center.trans(self.x, self.y).rot_rad(self.rotation).trans(-50.0, -50.0), g);
        });
    }

    fn on_input(&mut self, input: Input) {
   	    match input {
			Input::Press(but) => {
				match but {
					Button::Keyboard(Key::Up) => {
					    self.up_d = true;
					}
					Button::Keyboard(Key::Down) => {
					    self.down_d = true;
					}
					Button::Keyboard(Key::Left) => {
					    self.left_d = true;
					}
					Button::Keyboard(Key::Right) => {
					    self.right_d = true;
					}
					_ => {}
				}
			}
			Input::Release(but) => {
				match but {
					Button::Keyboard(Key::Up) => {
					    self.up_d = false;
					}
					Button::Keyboard(Key::Down) => {
					    self.down_d = false;
					}
					Button::Keyboard(Key::Left) => {
					    self.left_d = false;
					}
					Button::Keyboard(Key::Right) => {
					    self.right_d = false;
					}
					_ => {}
				}
			}
			_ => {}
		}
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
            Some(Event::Input(input))   => game.on_input(input),
            _ => {}
        }
    }
}
