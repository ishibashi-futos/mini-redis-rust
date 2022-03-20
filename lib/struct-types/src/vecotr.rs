#[derive(Debug, PartialEq)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub const ZERO: Vector2 = Vector2 { x: 0.0, y: 0.0 };
    pub const UNIT: Vector2 = Vector2 { x: 1.0, y: 0.0 };
    pub const NAME: &'static str = "Vector2";
    pub const ID: u32 = 18;

    pub fn scaled_by(self, scale: f32) -> Vector2 {
        Vector2 {
            x: self.x * scale,
            y: self.y * scale,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn use_zero() {
        assert_eq!(Vector2 { x: 0.0, y: 0.0 }, Vector2::ZERO);
    }

    #[test]
    fn use_unit() {
        assert_eq!(Vector2 { x: 1.0, y: 0.0 }, Vector2::UNIT);
    }

    #[test]
    fn scaled_unit() {
        let scaled = Vector2::UNIT.scaled_by(2.0);
        assert_eq!(Vector2 { x: 2.0, y: 0.0 }, scaled);
    }

    #[test]
    fn scaled_by() {
        let scaled = (Vector2 { x: 1.0, y: 2.0 }).scaled_by(2.0);
        assert_eq!(Vector2 { x: 2.0, y: 4.0 }, scaled);
    }
}
