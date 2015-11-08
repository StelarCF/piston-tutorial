use piston_window::*;
use gfx_device_gl::{Resources, Output, CommandBuffer};
use gfx_graphics::GfxGraphics;
use std::f64::consts::PI;

pub struct Object {
    x: f64,
    y: f64,
    rot: f64,
    sprite: Option<Texture<Resources>>,
    tur_x: f64,
    tur_y: f64,
    rot_tur: f64,
    turret: Option<Texture<Resources>>
}

#[allow(dead_code)]
impl Object {
    pub fn new() -> Object {
        Object {x : 0.0, y : 0.0, rot: 0.0, rot_tur: 0.0, tur_x: 0.0, tur_y: 0.0, sprite: None, turret: None}
    }
    pub fn mov(&mut self, x: f64, y: f64) {
        self.x += x;
        self.y += y;
    }
    pub fn mov_to(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }
    pub fn rot(&mut self, r: f64) {
        self.rot += r;
    }
    pub fn rot_to(&mut self, r: f64) {
        self.rot = r;
    }
    pub fn fwd(&mut self, d: f64) {
        self.x += d * (-self.rot.sin());
        self.y += d * self.rot.cos();
    }
    pub fn point_tur_to(&mut self, x: f64, y: f64) {
        self.tur_x = x;
        self.tur_y = y;
    }
    pub fn calc_tur_pos(&mut self, dt: f64) {
        let mut new_rot = (-(self.tur_x - self.x)).atan2(self.tur_y - self.y);
        if new_rot == self.rot_tur {
            return;
        }
        if new_rot < self.rot_tur && self.rot_tur - new_rot > new_rot + 2.0 * PI - self.rot_tur {
            new_rot += 2.0 * PI;
        }
        if new_rot > self.rot_tur && new_rot - self.rot_tur > self.rot_tur + 2.0 * PI - new_rot {
            new_rot -= 2.0 * PI;
        }
        let rot_speed = 1.0;
        if new_rot > self.rot_tur {
            if new_rot - self.rot_tur > rot_speed * dt {
                self.rot_tur += rot_speed * dt;
            } else {
                self.rot_tur = new_rot;
            }
        } else {
            if self.rot_tur - new_rot > rot_speed * dt {
                self.rot_tur -= rot_speed * dt;
            } else {
                self.rot_tur = new_rot;
            }
        }
        if self.rot_tur > 2.0 * PI {
            self.rot_tur -= 2.0 * PI;
        }
        if self.rot_tur < 0.0 {
            self.rot_tur += 2.0 * PI;
        }
    }
    pub fn update(&mut self, dt: f64) {
        self.calc_tur_pos(dt);
    }
    pub fn render(&self, g: &mut GfxGraphics<Resources, CommandBuffer<Resources>, Output>, view: math::Matrix2d) {
        match self.sprite {
            None => {
                let square = rectangle::square(0.0, 0.0, 100.0);
                let red = [1.0, 0.0, 0.0, 1.0];
                rectangle(red, square, view.trans(self.x, self.y).trans(-50.0, -50.0), g); // We translate the rectangle slightly so that it's centered; otherwise only the top left corner would be centered
            }
            Some(ref sprite) => {
                let (spritex, spritey) = sprite.get_size();
                let (ocx, ocy) = (spritex / 2, spritey / 2);
                image(sprite, view.trans(self.x, self.y).scale(0.75, 0.75).rot_rad(self.rot).trans(-(ocx as f64), -(ocy as f64)), g);
            }
        }
        match self.turret {
            None => {}
            Some(ref sprite) => {
                let (spritex, spritey) = sprite.get_size();
                let (ocx, ocy) = (spritex / 2, spritey / 2);
                image(sprite, view.trans(self.x, self.y).scale(0.75, 0.75).rot_rad(self.rot_tur).trans(-(ocx as f64), -(ocy as f64)), g);
            }
        }
    }
    pub fn set_sprite(&mut self, sprite: Texture<Resources>) {
        self.sprite = Some(sprite);
    }
    pub fn set_turret_sprite(&mut self, sprite: Texture<Resources>) {
        self.turret = Some(sprite);
    }
}
