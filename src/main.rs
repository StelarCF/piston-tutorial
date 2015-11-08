extern crate piston_window;
extern crate gfx_device_gl;
extern crate find_folder;
extern crate gfx_graphics;
extern crate gfx;

use piston_window::*;

mod object;
use object::Object;

struct Game {
    player: Object,
    up_d: bool, down_d: bool, left_d: bool, right_d: bool,
    scx: f64, scy: f64
}

impl Game {
    fn new() -> Game {
        Game { player : Object::new(), up_d: false, down_d: false, left_d: false, right_d: false, scx: 300.0, scy: 300.0 }
    }
    fn on_load(&mut self, w: &PistonWindow) {
        let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
        let tank_sprite = assets.join("E-100_Base.png");
        let tank_sprite = Texture::from_path(
                &mut *w.factory.borrow_mut(),
                &tank_sprite,
                Flip::None,
                &TextureSettings::new())
                .unwrap();
        let tank_turret = assets.join("E-100_Turret.png");
        let tank_turret = Texture::from_path(
                &mut *w.factory.borrow_mut(),
                &tank_turret,
                Flip::None,
                &TextureSettings::new())
                .unwrap();
        self.player.set_sprite(tank_sprite);
        self.player.set_turret_sprite(tank_turret);
    }
    fn on_update(&mut self, upd: UpdateArgs) {
        if self.up_d {
            //self.player.mov(0.0, -150.0 * upd.dt);
            self.player.fwd(150.0 * upd.dt);
        }
        if self.down_d {
            //self.player.mov(0.0, 150.0 * upd.dt);
            self.player.fwd(-150.0 * upd.dt);
        }
        if self.left_d {
            //self.player.mov(-150.0 * upd.dt, 0.0);
            self.player.rot(-1.0 * upd.dt);
        }
        if self.right_d {
            //self.player.mov(150.0 * upd.dt, 0.0);
            self.player.rot(1.0 * upd.dt);
        }
        self.player.update(upd.dt);
    }
    fn on_draw(&mut self, ren: RenderArgs, e: PistonWindow) {
        self.scx = (ren.width / 2) as f64;
        self.scy = (ren.height / 2) as f64;
        e.draw_2d(|c, g| {
            clear([0.8, 0.8, 0.8, 1.0], g);
            let center = c.transform.trans(self.scx, self.scy);
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
            Input::Move(mot) => {
                match mot {
                    Motion::MouseCursor(x, y) => {
                        self.player.point_tur_to(x - self.scx, y - self.scy);
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
