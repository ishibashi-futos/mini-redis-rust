/// ２つの自然数の最大公約数を求める - ユークリッドの互除法
pub fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);

    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_gcd() {
        let gcd = gcd(14, 15);

        assert_eq!(gcd, 1);
    }

    #[test]
    fn test_gcd_caluculated_values() {
        let gcd = gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19);

        let expected = 3 * 11 as u64;
        assert_eq!(gcd, expected);
    }

    #[test]
    fn test_run() {
        let x = 10;
        let y = 20;
        let actual = {
            x + y
        };

        assert_eq!(actual, 30);
    }
}
