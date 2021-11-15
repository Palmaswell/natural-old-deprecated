mod math;
pub use crate::math::Vector;
use rand::Rng;
use wasm_bindgen::prelude::*;

pub struct Mover {
    pub location: Vector,
    pub velocity: Vector,
    pub canvas_width: f64,
    pub canvas_height: f64,
}

impl Mover {
    fn constrain(&mut self) -> math::Vector {
        let mut x = self.location.x;
        let mut y = self.location.y;
        if x > self.canvas_width {
            x = 0.0
        } else if x < 0.0 {
            x = self.canvas_width
        }

        if y > self.canvas_height {
            y = 0.0
        } else if y < 0.0 {
            y = self.canvas_height
        }
        Vector { x, y }
    }
    pub fn update(&mut self) {
        self.location.add(&[1.0, 1.0]);
        self.constrain();
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log(a: u32, c: &str);
}

#[wasm_bindgen]
pub fn run(w: u32, h: u32) -> u32 {
    let mut rng = rand::thread_rng();
    let width = w as f64;
    let height = h as f64;
    let initial_location = Vector { x: 100.0, y: 500.0 };
    let initial_velocity = Vector {
        x: rng.gen_range(initial_location.x..(initial_location.x + 10.0)),
        y: rng.gen_range(0.0..500.0),
    };

    let mut mover = Mover {
        location: initial_location,
        velocity: initial_velocity,
        canvas_width: width,
        canvas_height: height,
    };

    // mover.velocity.add(&Vector {
    //     x: rng.gen_range(mover.location.x..(mover.location.x + 10.0)),
    //     y: rng.gen_range(0.0..500.0),
    // });
    mover.update();

    return mover.location.x as u32;
}
