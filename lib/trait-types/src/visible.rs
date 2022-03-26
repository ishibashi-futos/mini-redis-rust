use std::ops::{Add, Mul};

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

// Creatureを実装する型は、Visibleも実装していなければならない
pub trait Creature: Visible {}

#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Mul for Point {
    type Output = Point;

    fn mul(self, rhs: Point) -> Point {
        Point {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[test]
fn point_mul() {
    let point1 = Point { x: 10, y: 10 };
    let point2 = Point { x: 2, y: 3 };

    let actual = point1 * point2;

    assert_eq!(Point { x: 20, y: 30 }, actual);
}

#[allow(dead_code)]
fn cyclical_zip(v: Vec<u8>, u: Vec<u8>) -> impl Iterator<Item = u8> {
    v.into_iter().chain(u.into_iter()).cycle()
}

#[test]
#[ignore]
fn test_cyclical_zip() {
    let v = vec![10, 20, 30, 40];
    let u = vec![40, 30, 20, 10];

    // 無限ループになるので注意
    let _iterator = cyclical_zip(v, u);
}
