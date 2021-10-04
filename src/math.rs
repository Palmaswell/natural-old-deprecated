pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    pub fn add(&self, vector: &Vector) -> Vector {
        Vector {
            x: self.x + vector.x,
            y: self.y + vector.y,
        }
    }

    pub fn div(&self, number: &f64) -> Vector {
        Vector {
            x: self.x / number,
            y: self.y / number,
        }
    }

    pub fn magnitude(&self) -> f64 {
        let length: f64 = self.x * self.x + self.y * self.y;
        length.sqrt()
    }

    pub fn mult(&self, number: &f64) -> Vector {
        Vector {
            x: self.x * number,
            y: self.y * number,
        }
    }

    pub fn sub(&self, vector: &Vector) -> Vector {
        Vector {
            x: self.x - vector.x,
            y: self.y - vector.y,
        }
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
        let location = Vector { x: 13.0, y: 15.0 };
        let new_location = location.add(&Vector { x: 3.0, y: 5.5 });
        assert!(new_location.x == 16.0);
        assert!(new_location.y == 20.5);
    }

    #[test]
    fn should_substract_vectors() {
        let location = Vector { x: 1.0, y: 15.0 };
        let new_location = location.sub(&Vector { x: 5.0, y: 5.0 });
        assert!(new_location.x == -4.0);
        assert!(new_location.y == 10.0);
    }

    #[test]
    fn should_multiply_vectors() {
        let location = Vector { x: 14.0, y: 5.0 };
        let scaled_location = location.mult(&3.0);
        assert!(scaled_location.x == 42.0);
        assert!(scaled_location.y == 15.0);
    }

    #[test]
    fn should_divide_vectors() {
        let location = Vector { x: 14.0, y: 5.0 };
        let scaled_location = location.div(&2.0);
        assert!(scaled_location.x == 7.0);
        assert!(scaled_location.y == 2.5);
    }

    #[test]
    fn should_calculate_magnitute() {
        let location = Vector { x: 10.0, y: 5.0 };
        let magnitude = location.magnitude();
        assert!(magnitude == 11.180339887498949);
    }
}
