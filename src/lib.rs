mod math;
pub use crate::math::Vector;
use rand::Rng;
use wasm_bindgen::prelude::*;

pub struct Mover {
    pub location: Vector,
    pub velocity: Vector,
    pub canvas_width: u32,
    pub canvas_height: u32,
}

pub struct MoverResult(pub u8, pub u8);

impl Mover {
    fn constrain(&mut self) -> math::Vector {
        let mut x = self.location.x;
        let mut y = self.location.y;
        if x > self.canvas_width as f64 {
            x = 0.0
        } else if x < 0.0 {
            x = self.canvas_width as f64
        }

        if y > self.canvas_height as f64 {
            y = 0.0
        } else if y < 0.0 {
            y = self.canvas_height as f64
        }
        Vector { x, y }
    }
    pub fn update(&mut self) {
        self.location = self.location.add(&self.velocity);
        self.constrain();
    }
}

#[wasm_bindgen]
pub fn run(width: u32, height: u32) -> u32 {
    let mut rng = rand::thread_rng();
    let initial_location = Vector { x: 100.0, y: 500.0 };
    let initial_velocity = Vector {
        x: rng.gen_range(-2.0..2.0),
        y: rng.gen_range(-2.0..2.0),
    };

    let mut mover = Mover {
        location: initial_location,
        velocity: initial_velocity,
        canvas_width: width,
        canvas_height: height,
    };

    loop {
        mover.velocity.add(&Vector {
            x: rng.gen_range(-2.0..2.0),
            y: rng.gen_range(-2.0..2.0),
        });
        mover.update();
        println!("x {} y {}", mover.location.x, mover.location.y);

        return mover.location.x as u32;
    }
}
