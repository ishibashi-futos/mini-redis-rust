use std::io::Write;

pub fn say_hello(out: &mut dyn Write) -> std::io::Result<()> {
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
