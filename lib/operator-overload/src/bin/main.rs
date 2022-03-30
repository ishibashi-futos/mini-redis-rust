use std::ops::*;
fn main() {
    println!("Add: 4.125 + 5.75 = {}", 4.125.add(5.75));

    println!("Sub: 9.875 - 5.75 = {}", 9.875.sub(5.75));

    println!("Mul: 4.0 * 2 = {}", 4.0.mul(2.0));

    println!("Div: 4 / 2 = {}", 4.0.div(2.0));

    println!("Rem: {}", 5.rem(2));

    println!("BitAnd: 5 & 10 = {}", 5 & 10);

    println!("BitOr: 5 | 10 = {}", 5 | 10);

    println!("BitXor: 5 ^ 10 = {}", 5 ^ 10);

    println!("Shl: 5 << 10 = {}", 5 << 10);

    println!("Shr: 5 >> 10 = {}", 5 >> 10);

    println!("{}", 100.neg());
    println!("{}", 100.not());
}
