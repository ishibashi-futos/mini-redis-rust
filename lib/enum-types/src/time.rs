#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Months,
    Years,
}

impl TimeUnit {
    pub fn to_str(self) -> &'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years",
        }
    }

    pub fn singular(self) -> &'static str {
        self.to_str().trim_end_matches('s')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_str() {
        let seconds = TimeUnit::Seconds.to_str();

        assert_eq!("seconds", seconds);
    }

    #[test]
    fn singular() {
        let years = TimeUnit::Years;
        let singular = years.singular();

        assert_eq!("year", singular);
    }
}

// データを保持する列挙型も作成できる
#[derive(PartialEq, Debug)]
pub enum RoughTime {
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32),
}

impl RoughTime {
    pub fn rough_time_to_english(&self) -> String {
        match self {
            &RoughTime::InThePast(units, 1) => format!("a {} ago", units.singular()),
            &RoughTime::InThePast(units, count) => format!("{} {} ago", count, units.to_str()),
            &RoughTime::JustNow => format!("just now"),
            &RoughTime::InTheFuture(units, 1) => format!("a {} from now", units.singular()),
            &RoughTime::InTheFuture(units, count) => {
                format!("{} {} from now", count, units.to_str())
            }
        }
    }
}

#[cfg(test)]
mod tests_rough_time {
    use super::*;

    #[test]
    fn four_score_and_seven_years_ago() {
        let time = RoughTime::InThePast(TimeUnit::Years, 4 * 20 + 7);

        assert_eq!(RoughTime::InThePast(TimeUnit::Years, 87), time);
    }

    #[test]
    fn three_hours_from_now() {
        let time = RoughTime::InTheFuture(TimeUnit::Hours, 3);

        assert_eq!(RoughTime::InTheFuture(TimeUnit::Hours, 3), time);
    }

    #[test]
    fn future_to_english() {
        let time = RoughTime::InTheFuture(TimeUnit::Hours, 3);

        let actual = time.rough_time_to_english();

        assert_eq!("3 hours from now".to_string(), actual);
    }

    #[test]
    fn future_to_english_singular() {
        let time = RoughTime::InTheFuture(TimeUnit::Hours, 1);

        let actual = time.rough_time_to_english();

        assert_eq!("a hour from now".to_string(), actual);
    }
}

#[derive(PartialEq, Debug, PartialOrd)]
pub struct Point3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, PartialEq)]
pub enum Shape {
    Sphare { center: Point3d, radius: f32 },
    Cuboid { corner1: Point3d, corner2: Point3d },
}

impl Point3d {
    pub const ORIGIN: Point3d = Point3d {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
}

#[cfg(test)]
mod tests_shape {
    use super::{Point3d, Shape};

    #[test]
    fn unit_sphere() {
        let shape = Shape::Sphare {
            center: Point3d::ORIGIN,
            radius: 1.0,
        };

        match shape {
            Shape::Sphare {
                center: point,
                radius: value,
            } => {
                assert_eq!(0.0, point.x);
                assert_eq!(0.0, point.y);
                assert_eq!(0.0, point.z);
                assert_eq!(1.0, value);
            }
            _ => panic!("not expected"),
        }
    }

    #[test]
    fn unit_cuboid() {
        let shape = Shape::Cuboid {
            corner1: Point3d::ORIGIN,
            corner2: Point3d {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
        };

        match shape {
            Shape::Cuboid {
                corner1: point1,
                corner2: point2,
            } if point1.x == 0.0 && point1.y == 0.0 && point1.z == 0.0 => {
                assert_eq!(1.0, point2.x);
                assert_eq!(2.0, point2.y);
                assert_eq!(3.0, point2.z);
            }
            _ => panic!("not expected"),
        }
    }
}

pub fn describe_point(x: i32, y: i32) -> &'static str {
    use std::cmp::Ordering::*;

    match (x.cmp(&0), y.cmp(&0)) {
        (Equal, Equal) => "at the origin",
        (_, Equal) => "on the x axis",
        (Equal, _) => "on the y axis",
        (Greater, Greater) => "in the first quadrant",
        (Less, Greater) => "in the second quadrant",
        _ => "somwhere else",
    }
}

#[test]
fn at_the_origin() {
    let actual = describe_point(0, 0);
    assert_eq!("at the origin", actual);
}

pub struct Point {
    pub x: u32,
    pub y: u32,
}

#[test]
fn origin() {
    let point = Point { x: 0, y: 0 };

    match point {
        Point { x: 0, y: 0 } => assert!(true),
        #[allow(unused_variables)]
        Point { x, y } => panic!("Illegal pattern"),
    };
}
