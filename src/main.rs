extern crate piston_window;

use piston_window::*;

fn main() {
    let window: PistonWindow = WindowSettings::new(
        "piston-tutorial",
        [600, 600]
    )
    .exit_on_esc(true)
    .build()
    .unwrap();
    let mut rotation: f64 = 0.0;
    for e in window {
        match e.event {
            Some(Event::Update(UpdateArgs { dt })) => {
                rotation += 3.0 * dt;
            }
            _ => {

            }
        }
        e.draw_2d(|c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            let center = c.transform.trans(300.0, 300.0);
            let square = rectangle::square(0.0, 0.0, 100.0);
            let red = [1.0, 0.0, 0.0, 1.0];
            rectangle(red, square, center.rot_rad(rotation).trans(-50.0, -50.0), g); // We translate the rectangle slightly so that it's centered; otherwise only the top left corner would be centered
        });
    }
}
