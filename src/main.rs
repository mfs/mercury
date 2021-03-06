extern crate piston_window;
extern crate gfx_device_gl;
extern crate gfx_graphics;
extern crate gfx;

#[macro_use]
extern crate cfor;

mod starfield;
use starfield::get_stars;

use piston_window::*;
use gfx_device_gl::Resources;
use std::path::Path;
use std::collections::HashSet;

// much nicer way to handle keyboard from:
// github.com/caspark/2015-08-rust-half-a-game-piston-talk
struct KeyState {
    held_keys: HashSet<Button>,
}

impl KeyState {
    fn new() -> Self {
        KeyState { held_keys: HashSet::new() }
    }

    fn update(&mut self, input: &Input ) {
        match input {
            &Input::Press(x)   => { self.held_keys.insert(x); },
            &Input::Release(x) => { self.held_keys.remove(&x); },
            _                  => {},
        };
    }

    fn is_down(&self, button: &Button) -> bool {
        self.held_keys.contains(button)
    }
}

struct Game {
    rotation: f64,
    x: f64, y: f64,
    vx: f64, vy: f64,
    fuel: f64,
    keystate: KeyState,
    image: Option<Texture<Resources>>
}

impl Game {
    fn new() -> Game {
        Game {
            rotation: 0.0, x: 0.0, y: 0.0,
            vx: 0.0, vy: 0.0,
            fuel: 200.0,
            keystate: KeyState::new(),
            image: None
        }
    }

    fn on_load(&mut self, w: &PistonWindow) {
        let texture = Texture::from_path(
            &mut *w.factory.borrow_mut(),
            &Path::new("assets/fighterspr1.png"),
            Flip::None,
            &TextureSettings::new()
        ).unwrap();

	self.image = Some(texture);
    }

    fn on_update(&mut self, update: UpdateArgs) {
        //self.rotation += 0.7 * update.dt;

        self.x += self.vx;
        self.y += self.vy;

        println!("fuel = {}", self.fuel);

        if self.fuel <= 0.0 {
            return;
        }

        if self.keystate.is_down(&Button::Keyboard(Key::Up) ) {
            self.vy += (-5.0) * update.dt;
            self.fuel -= 10.0 * update.dt;
        }
        if self.keystate.is_down(&Button::Keyboard(Key::Down) ) {
            self.vy += (5.0) * update.dt;
            self.fuel -= 10.0 * update.dt;
        }
        if self.keystate.is_down(&Button::Keyboard(Key::Left) ) {
            self.vx += (-5.0) * update.dt;
            self.fuel -= 10.0 * update.dt;
        }
        if self.keystate.is_down(&Button::Keyboard(Key::Right) ) {
            self.vx += (5.0) * update.dt;
            self.fuel -= 10.0 * update.dt;
        }


    }

    fn on_draw(&mut self, render: RenderArgs, e: PistonWindow) {
        e.draw_2d(|c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            let center = c.transform.trans((render.width / 2) as f64, (render.height / 2) as f64);
            let square = rectangle::square(0.0, 0.0, 100.0);
            let gray = [0.7, 0.7, 0.7, 1.0];
            //rectangle(gray, square, center.trans(self.x, self.y).rot_rad(self.rotation).trans(-50.0, -50.0), g);
            match self.image {
                Some(ref x) => {
                    let (width, height) = x.get_size();
                    let t = center.trans(self.x, self.y).rot_rad(self.rotation)
                                  .trans(-(width as f64) * 0.2 /2.0, -(height as f64) * 0.2 /2.0)
                                  .scale(0.2, 0.2);
                    image(x, t, g);
                },
                _ => {}
            }
        });
    }

    fn on_input(&mut self, input: Input) {
        self.keystate.update(&input);
    }
}

fn main() {

    get_stars(0.0, 0.0, 300.0, 300.0);
    return;

    let window: PistonWindow = WindowSettings::new("mercury", [600, 600])
        .exit_on_esc(true).build().unwrap();

    let mut game = Game::new();

    game.on_load(&window);


    for e in window {
        match e.event {
            Some(Event::Update(update)) => game.on_update(update),
            Some(Event::Render(render)) => game.on_draw(render, e),
            Some(Event::Input(input))   => game.on_input(input),
            _ => {}
        }
    }
}
