#[test]
fn drain() {
    let mut outher = "Earth".to_string();
    let inner = String::from_iter(outher.drain(0..2));

    assert_eq!("Ea", inner);
    assert_eq!("rth", outher);
}

#[test]
fn arr() {
    let mut v = vec![1, 2, 3, 4, 5];
    let v2 = v.drain(2..=3).collect::<Vec<i32>>();

    assert_eq!(vec![3, 4], v2);
    assert_eq!(vec![1, 2, 5], v);
}
