extern crate piston_window;
extern crate gfx_device_gl;
extern crate find_folder;
extern crate gfx_graphics;
extern crate gfx;

use piston_window::*;

mod object;
use object::Object;

struct Game {
    rotation: f64,
    player: Object,
    up_d: bool, down_d: bool, left_d: bool, right_d: bool
}

impl Game {
    fn new() -> Game {
        Game { rotation : 0.0, player : Object::new(), up_d: false, down_d: false, left_d: false, right_d: false }
    }
    fn on_load(&mut self, w: &PistonWindow) {
        let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
        let tank_sprite = assets.join("E-100_preview.png");
        let tank_sprite = Texture::from_path(
                &mut *w.factory.borrow_mut(),
                &tank_sprite,
                Flip::None,
                &TextureSettings::new())
                .unwrap();
        self.player.setSprite(tank_sprite);
    }
    fn on_update(&mut self, upd: UpdateArgs) {
        self.rotation += 3.0 * upd.dt;
        if self.up_d {
            self.player.mov(0.0, -150.0 * upd.dt);
        }
        if self.down_d {
            self.player.mov(0.0, 150.0 * upd.dt);
        }
        if self.left_d {
            self.player.mov(-150.0 * upd.dt, 0.0);
        }
        if self.right_d {
            self.player.mov(150.0 * upd.dt, 0.0);
        }
    }
    fn on_draw(&mut self, ren: RenderArgs, e: PistonWindow) {
        e.draw_2d(|c, g| {
            clear([0.8, 0.8, 0.8, 1.0], g);
            let center = c.transform.trans((ren.width / 2) as f64, (ren.height / 2) as f64);
            self.player.render(g, center);
        });
    }
    fn on_input(&mut self, inp: Input) {
        match inp {
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
    let window: PistonWindow = WindowSettings::new(
        "piston-tutorial",
        [600, 600]
    )
    .exit_on_esc(true)
    .build()
    .unwrap();
    let mut game = Game::new();
    game.on_load(&window);
    for e in window {
        match e.event {
            Some(Event::Update(upd)) => {
                game.on_update(upd);
            }
            Some(Event::Render(ren)) => {
                game.on_draw(ren, e);
            }
            Some(Event::Input(inp)) => {
                game.on_input(inp);
            }
            _ => {

            }
        }
    }
}
