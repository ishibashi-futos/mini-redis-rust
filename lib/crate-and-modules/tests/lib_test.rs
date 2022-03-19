use crate_and_modules::Fern;

#[test]
fn grow_once() {
    let mut f = Fern { size: 1.0, growth_rate: 1.15 };

    f.grow();

    assert_eq!(1.0 * 1.15, f.size, "macos以外の場合エラーになるだろう");
}

#[test]
fn grow_twice() {
    let mut f = Fern { size: 1.0, growth_rate: 1.15 };

    f.grow();
    f.grow();

    assert_eq!(1.0 * 1.15 * 1.15, f.size, "macos以外の場合エラーになるだろう");
}
