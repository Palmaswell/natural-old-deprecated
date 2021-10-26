mod math;
pub use crate::math::Vector;

pub struct Mover {
    pub location: Vector,
    pub velocity: Vector,
    pub canvas_width: u32,
    pub canvas_height: u32,
}

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
