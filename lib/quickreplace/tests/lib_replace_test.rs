use quickreplace::*;

#[test]
fn replace_valid_regex_pattern() -> Result<(), regex::Error> {
    let actual = replace("world", "Rust", "Hello, world")?;


    assert_eq!("Hello, Rust", actual);
    Ok(())
}

#[test]
#[should_panic(expected = "should panic!")]
fn replace_invalid_regex_pattern() {

    match replace("[[a-z]", "Rust", "Hello, world") {
        Err(_) => panic!("should panic!"),
        _ => {}
    }
    // panicして終了するはずなのでこの行は呼ばれない
}
