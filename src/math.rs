pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    pub fn add(&mut self, coordinate: &[f64; 2]) {
        let vector = add_vector(self, coordinate);
        self.x = vector.x;
        self.y = vector.y;
    }

    pub fn div(&mut self, divisor: &f64) {
        let vector = div_vector(self, divisor);
        self.x = vector.x;
        self.y = vector.y;
    }

    pub fn magnitude(&self) -> f64 {
        let length: f64 = self.x * self.x + self.y * self.y;
        calc_precision(&length.sqrt())
    }

    pub fn normalize(&mut self) {
        let magnitude = self.magnitude();
        // Return a new Vector with the initial values in case
        // the magnitude is 0.0
        if magnitude > 0.0 {
            self.div(&magnitude);
        }
    }

    pub fn mult(&mut self, multiplicant: &f64) {
        let vector = mult_vector(self, multiplicant);
        self.x = vector.x;
        self.y = vector.y;
    }

    pub fn sub(&mut self, coordinate: &[f64; 2]) {
        let vector = sub_vector(self, coordinate);
        self.x = vector.x;
        self.y = vector.y;
    }
}

fn calc_precision(value: &f64) -> f64 {
    let precision = 100_000_000.0;
    (value * precision).round() / precision
}

fn add_vector(vector: &mut Vector, coordinate: &[f64; 2]) -> Vector {
    Vector {
        x: calc_precision(&(vector.x + coordinate[0])),
        y: calc_precision(&(vector.y + coordinate[1])),
    }
}

fn sub_vector(vector: &mut Vector, coordinate: &[f64; 2]) -> Vector {
    Vector {
        x: calc_precision(&(vector.x - coordinate[0])),
        y: calc_precision(&(vector.y - coordinate[1])),
    }
}

fn div_vector(vector: &mut Vector, divisor: &f64) -> Vector {
    Vector {
        x: calc_precision(&(vector.x / divisor)),
        y: calc_precision(&(vector.y / divisor)),
    }
}

fn mult_vector(vector: &mut Vector, multiplicant: &f64) -> Vector {
    Vector {
        x: calc_precision(&(vector.x * multiplicant)),
        y: calc_precision(&(vector.y * multiplicant)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn instantiate_vector() {
        let location = Vector { x: 3.0, y: 5.5 };
        assert!(location.x == 3.0);
        assert!(location.y == 5.5);
    }

    #[test]
    fn should_add_vectors() {
        let mut location = Vector { x: 13.0, y: 15.0 };
        assert!(location.x == 13.0);
        assert!(location.y == 15.0);

        location.add(&[3.0, 5.5]);
        assert!(location.x == 16.0);
        assert!(location.y == 20.5);
    }

    #[test]
    fn should_substract_vectors() {
        let mut location = Vector { x: 1.0, y: 15.0 };
        assert!(location.x == 1.0);
        assert!(location.y == 15.0);

        location.sub(&[5.0, 5.0]);
        assert!(location.x == -4.0);
        assert!(location.y == 10.0);
    }

    #[test]
    fn should_multiply_vectors() {
        let mut location = Vector { x: 14.0, y: 5.0 };
        location.mult(&3.0);
        assert!(location.x == 42.0);
        assert!(location.y == 15.0);
    }

    #[test]
    fn should_divide_vectors() {
        let mut location = Vector { x: 14.0, y: 5.0 };
        assert!(location.x == 14.0);
        assert!(location.y == 5.0);
        location.div(&2.0);
        assert!(location.x == 7.0);
        assert!(location.y == 2.5);
    }

    #[test]
    fn should_calculate_magnitute() {
        let location = Vector { x: 10.0, y: 5.0 };
        let magnitude = location.magnitude();
        assert!(magnitude == 11.18033989);
    }

    #[test]
    fn should_normalize() {
        let mut location = Vector { x: 10.0, y: 5.0 };
        location.normalize();
        assert!(location.x == 0.89442719);
        assert!(location.y == 0.4472136);
    }
}
