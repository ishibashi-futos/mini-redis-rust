#[derive(Debug, PartialEq)]
pub enum Ordering {
    Less,
    Equal,
    Greater,
}

pub fn compare(n: i32, m: i32) -> Ordering {
    if n < m {
        Ordering::Less
    } else if n > m {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

#[cfg(test)]
mod tests {
    use super::{
        compare,
        Ordering::{self, *},
    };

    #[test]
    fn less() {
        assert_eq!(Less, compare(0, 1));
    }

    #[test]
    fn greater() {
        assert_eq!(Greater, compare(1, 0));
    }

    #[test]
    fn equal() {
        assert_eq!(Ordering::Greater, compare(1, 0));
    }
}
