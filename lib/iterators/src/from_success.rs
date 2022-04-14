use std::ops::{Add, Mul};

#[cfg(test)]
mod tests {
    use super::*;
    use expect_rs::expect;
    use rand::random;
    use std::iter::{from_fn, successors};

    #[test]
    fn a() {
        let length: Vec<f64> = from_fn(|| Some(random::<f64>() - random::<f64>().abs()))
            .take(1000)
            .collect();

        assert_eq!(length.len(), 1000);
    }

    #[test]
    fn s() {
        let c = Complex { re: 1.0, mi: 2.0 };
        let zero = Complex { re: 0.0, mi: 0.0 };
        let s = successors(Some(zero), |&z| Some(z * z + c))
            .take(10)
            .enumerate()
            .find(|(_i, z)| z.norm_sqr() > 4.0)
            .map(|(i, _z)| i);
        expect(&s).equals(&None);
    }
}

#[derive(Clone, Copy)]
struct Complex {
    re: f64,
    mi: f64,
}

impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            mi: self.mi + rhs.mi,
        }
    }
}

impl Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            mi: self.mi + rhs.mi,
        }
    }
}

impl Complex {
    #[allow(dead_code)]
    fn norm_sqr(&self) -> f64 {
        rand::random::<f64>()
    }
}
