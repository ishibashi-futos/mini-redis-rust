use expect::expect;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn index() {
    let mut m = HashMap::new();
    m.insert("十", 10);
    m.insert("百", 100);
    m.insert("千", 1000);

    expect(&m["十"]).equals(&10);
    expect(&m["千"]).equals(&1000);
    expect(&m).contains_key(&"十");

    use std::ops::Index;

    expect(&*m.index("十")).equals(&10);
    expect(&*m.index("千")).equals(&1000);
}

#[test]
fn is_ok() {
    index();
}

#[allow(dead_code)]
struct Image<P> {
    width: usize,
    pixels: Vec<P>,
}

impl<P: Default + Copy> Image<P> {
    #[allow(dead_code)]
    fn new(width: usize, height: usize) -> Image<P> {
        Image {
            width,
            pixels: vec![P::default(); width * height],
        }
    }
}

impl<P> std::ops::Index<usize> for Image<P> {
    type Output = [P];

    fn index(&self, row: usize) -> &[P] {
        let start = row * self.width;
        &self.pixels[start..start + self.width]
    }
}
