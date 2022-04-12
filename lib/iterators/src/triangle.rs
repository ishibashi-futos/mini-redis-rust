pub fn triangle(n: i32) -> i32 {
    (1..=n).fold(0, |sum, item| sum + item)
}

#[cfg(test)]
mod tests {
    use crate::triangle::*;
    use expect_rs::expect;

    #[test]
    fn triangle_test() {
        let actual = triangle(10);

        expect(&actual).equals(&55);
    }

    #[test]
    fn iterators_test() {
        let v = vec![0, 1, 2, 3, 4, 5];

        // 以下のコードは、次のコードを省略しているに過ぎない
        for element in &v {
            expect(element).in_range(0..6);
        }

        // 実際にはNextメソッドが呼ばれ、Nextがなくなるまで繰り返す
        let mut iter = (&v).into_iter();
        while let Some(element) = iter.next() {
            expect(element).in_range(0..6);
        }
    }
}
