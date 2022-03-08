use quickreplace::*;

#[test]
fn parse_args_length_less_than_4() {
    let args = vec![String::from("args1"), String::from("args2"), String::from("args1")];

    let actual = parse_args(args);

    assert_eq!(Err(3), actual);
}

#[test]
fn parse_args_length_0_is_fail() {
    let args: Vec<String> = Vec::new();

    let actual = parse_args(args);

    assert_eq!(Err(0), actual);
}

#[test]
fn parse_args_is_ok() {
    let args = vec![
        String::from("world"),
        String::from("Rust"),
        String::from("test.txt"),
        String::from("test_modified.txt"),
    ];

    let actual = parse_args(args);

    assert_eq!(Ok(Arguments{
        target: String::from("world"),
        replacement: String::from("Rust"),
        filename: String::from("test.txt"),
        output: String::from("test_modified.txt"),
    }), actual);
}
