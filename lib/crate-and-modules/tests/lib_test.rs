use std::num::ParseIntError;

use crate_and_modules::Fern;

#[test]
fn grow_once() {
    let mut f = Fern {
        size: 1.0,
        growth_rate: 1.15,
    };

    f.grow();

    assert_eq!(1.0 * 1.15, f.size, "macos以外の場合エラーになるだろう");
}

#[test]
fn grow_twice() {
    let mut f = Fern {
        size: 1.0,
        growth_rate: 1.15,
    };

    f.grow();
    f.grow();

    assert_eq!(
        1.0 * 1.15 * 1.15,
        f.size,
        "macos以外の場合エラーになるだろう"
    );
}

#[test]
#[allow(unconditional_panic, unused_must_use)]
#[should_panic(expected = "divide by zero")]
fn test_divide_by_zero_error() {
    1 / 0;
}

#[test]
fn explicit_radix() -> Result<(), ParseIntError> {
    i32::from_str_radix("1024", 10)?;
    Ok(())
}
