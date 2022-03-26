pub trait Vegetable {
    fn name(&self) -> &'static str;
}

pub struct Salad {
    pub veggies: Vec<Box<dyn Vegetable>>,
}

impl Salad {
    pub fn new(veggies: Vec<Box<dyn Vegetable>>) -> Salad {
        Salad { veggies }
    }
}

pub struct Lettuce {}

impl Vegetable for Lettuce {
    fn name(&self) -> &'static str {
        "Lettuce"
    }
}

#[test]
fn order_salad() {
    let salad = Salad::new(vec![Box::new(Lettuce {})]);

    let lettuce = salad.veggies.get(0).unwrap();

    assert_eq!("Lettuce", lettuce.name());
}
