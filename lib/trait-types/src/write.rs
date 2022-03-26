use std::{fmt::Debug, hash::Hash, io::Write};

pub fn say_hello<W: Write>(out: &mut W) -> std::io::Result<()> {
    out.write_all(b"Hello, World\n")?;
    out.flush()
}

#[test]
fn write_bytes() {
    let mut bytes = vec![];
    say_hello(&mut bytes).unwrap();

    let mut out = vec![];
    out.write(b"Hello, World\n").unwrap();

    assert_eq!(out, bytes);
}

pub fn top_ten<T>(values: &mut Vec<T>) -> Vec<T>
where
    T: Debug + Hash + Ord + Clone,
{
    let mut vec = Vec::with_capacity(10);

    values.sort();

    for i in 0..10 {
        match values.get(i) {
            Some(value) => vec.push(value.clone()),
            None => break,
        }
    }

    vec
}

#[test]
fn top_ten_test() {
    let mut values = (1..100).collect::<Vec<u32>>();

    let actual = top_ten(&mut values);

    assert_eq!(99, values.len());
    assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], actual);
}

#[allow(dead_code)]
fn dot_product<const N: usize>(a: [f64; N], b: [f64; N]) -> f64 {
    let mut sum = 0.;

    for i in 0..N {
        sum += a[i] * b[i];
    }

    sum
}
