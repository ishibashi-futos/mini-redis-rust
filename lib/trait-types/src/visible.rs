pub struct Canvas {}

impl Canvas {
    pub fn write_at(&self, _x: i32, _y: i32, _c: char) {}
}

pub trait Visible {
    fn draw(&self, canvas: &mut Canvas);
    fn hit_test(&self, x: i32, y: i32) -> bool;
}

pub struct Broom {
    pub x: i32,
    pub y: i32,
    pub height: i32,
    pub width: i32,
}

impl Broom {
    pub const ORIGIN: Broom = Broom {
        x: 0,
        y: 0,
        height: 0,
        width: 0,
    };
}

impl Visible for Broom {
    fn draw(&self, canvas: &mut Canvas) {
        for y in self.y - self.height..self.y {
            canvas.write_at(self.y, y, '|');
        }
        canvas.write_at(self.x, self.y, 'M');
    }

    fn hit_test(&self, x: i32, y: i32) -> bool {
        self.x == x && self.y - self.height - 1 <= y && y <= self.y
    }
}
