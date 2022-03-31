use std::cmp::{Ordering, PartialOrd};

#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.x == other.y && self.y == other.y {
            return Some(Ordering::Equal)
        } else if self.x >= other.x && self.y >= other.y {
            return Some(Ordering::Greater)
        } else if self.x <= other.x && self.y <= other.y {
            return Some(Ordering::Less)
        }
        None
    }
}

#[test]
fn equal() {
    let point1 = Point { x: 10.0, y: 10.0 };
    let point2 = Point { x: 10.0, y: 10.0 };

    assert_eq!(point1, point2)
}

#[test]
fn greater() {
    let point1 = Point { x: 10.0, y: 10.0 };
    let point2 = Point { x: 9.999999, y: 9.999999 };

    assert!(point1 > point2);
}

#[test]
fn less() {
    let point1 = Point { x: 10.0, y: 10.0 };
    let point2 = Point { x: 10.1, y: 10.1 };

    assert!(point1 <= point2);
}
