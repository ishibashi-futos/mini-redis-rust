use std::ops::Add;

pub trait Greet {
    // traitの関連定数を設定することができる
    const GREETING: &'static str = "Hello";

    fn greet(&self) -> String {
        format!("{}", Self::GREETING)
    }
}

pub struct Greeter {}

impl Greet for Greeter {}

#[test]
fn greet() {
    let greeter = Greeter {};

    assert_eq!("Hello", greeter.greet());
}

pub struct Greeter2 {}

impl Greet for Greeter2 {
    // `Self::変数名`実装クラスから呼び出しが可能
    fn greet(&self) -> String {
        format!("{}, World!", Self::GREETING)
    }
}

#[test]
fn greet2() {
    let greeter = Greeter2 {};

    assert_eq!("Hello, World!", greeter.greet());
}

pub struct Greeter3 {}

impl Greet for Greeter3 {
    // Overrideも可能
    const GREETING: &'static str = "Hallo";
    fn greet(&self) -> String {
        format!("{}, World!", Self::GREETING)
    }
}

impl Greeter3 {
    pub fn greet_super(&self) -> String {
        format!("{}, World!", Self::GREETING)
    }
}

#[test]
fn greet3() {
    let greeter = Greeter3 {};

    assert_eq!("Hallo, World!", greeter.greet());
}

#[test]
fn greet3_super() {
    let greeter = Greeter3 {};

    assert_eq!("Hallo, World!", greeter.greet_super());
}

// 宣言するだけで値を指定しないこともできる
trait Float {
    const ZERO: Self;
    const ONE: Self;
}

impl Float for f32 {
    const ZERO: f32 = 0.0f32;
    const ONE: f32 = 1.0f32;
}

impl Float for f64 {
    const ZERO: f64 = 0.0f64;
    const ONE: f64 = 1.0f64;
}

// ジェネリックのコード内でこれらの定数を使用できる
#[allow(dead_code)]
fn add_one<T: Float + Add<Output = T>>(value: T) -> T {
    value + T::ONE
}

#[test]
fn add_one_f32() {
    let value: f32 = 0.1;

    let actual = add_one(value);

    assert_eq!(1.1, actual);
}

#[test]
fn add_one_f64() {
    let value: f64 = 0.1;

    let actual = add_one(value);

    assert_eq!(1.1, actual);
}

pub fn dot<const Z: usize>(v1: &[i64; Z], v2: &[i64; Z]) -> i64 {
    let mut total: i64 = 0;

    for i in 0..v1.len() {
        total = total + v1[i] * v2[i];
    }
    total
}

#[test]
fn sum_dot() {
    let v1 = [1; 10];
    let v2 = [2; 10];

    let actual = dot(&v1, &v2);

    assert_eq!(20, actual);
}
