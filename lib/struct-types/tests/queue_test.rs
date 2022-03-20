use struct_types::queue::*;

#[test]
fn two_chars() {
    let mut q = Queue::new();

    q.push('1');
    q.push('∞');
    q.pop();

    assert_eq!(Some('∞'), q.pop())
}

#[test]
fn is_empty() {
    let q = Queue::new();

    assert!(q.is_empty());
}

#[test]
fn is_not_empty() {
    let mut q = Queue::new();

    q.push('c');

    assert!(!q.is_empty());
}

#[test]
fn already_poped_be_empty() {
    let mut q = Queue::new();

    q.push('a');
    q.push('b');
    q.push('c');
    q.push('d');
    q.push('e');
    flush(&mut q);

    assert!(q.is_empty());
}

/// test utility fn -> Queueを空っぽにする
fn flush(q: &mut Queue) {
    while let Some(_) = q.pop() {
        // とにかく値をpopする
    }
}

#[test]
fn split() {
    let mut q = Queue::new();

    q.push('a');
    q.push('b');
    q.pop();

    q.push('x');

    let (older, younger) = q.split();
    assert_eq!(vec!['b'], older);
    assert_eq!(vec!['x'], younger);
}

#[test]
fn is_empty_owned() {
    #[allow(unused_mut)]
    let mut q = Queue::new();
    assert!(q.is_empty_owned());
    // q.pop(); // moveされているのでこの行はコンパイルできない
}
