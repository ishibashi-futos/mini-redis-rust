pub struct Broom {
    pub name: String,
    pub height: u32,
    pub health: u32,
    pub position: (f32, f32, f32),
    pub intent: BroomIntent,
}

#[derive(Debug, Copy, Clone)]
pub enum BroomIntent {
    FetchWater,
    DumpWater,
}

pub fn chop(b: Broom) -> (Broom, Broom) {
    let mut broom1 = Broom {
        height: b.height / 2,
        ..b
    };
    let mut broom2 = Broom {
        name: broom1.name.clone(),
        ..broom1
    };

    broom1.name.push_str(" I");
    broom2.name.push_str(" II");

    (broom1, broom2)
}

#[cfg(test)]
mod test {
    use crate::broom::*;

    fn new_hokey() -> Broom {
        Broom {
            name: "Hokey".to_string(),
            height: 60,
            health: 100,
            position: (100.0, 200.0, 0.0),
            intent: BroomIntent::FetchWater,
        }
    }

    #[test]
    fn it_work() {
        let hokey = new_hokey();
        let (hokey1, hokey2) = chop(hokey);

        assert_eq!("Hokey I", hokey1.name);
        assert_eq!(30, hokey1.height);
        assert_eq!(100, hokey1.health);

        assert_eq!("Hokey II", hokey2.name);
        assert_eq!(30, hokey2.height);
        assert_eq!(100, hokey2.health);
    }
}

/// タプル構造体
#[allow(dead_code)]
struct Bounds(usize, usize);
#[allow(dead_code)]
impl Bounds {
    fn pixels(&self) -> usize {
        self.0 * self.1
    }
}

#[test]
fn tuple_struct_it_work() {
    let bounds = Bounds(1024, 768);

    assert_eq!(1024, bounds.0);
    assert_eq!(768, bounds.1);
}

#[test]
fn tuple_struct_fn_it_work() {
    let bounds = Bounds(1024, 768);

    assert_eq!(786432, bounds.pixels());
}

// タプル構造体はラップ型を作るのにも便利
#[allow(dead_code)]
struct Ascii(Vec<u8>);

#[test]
fn tuple_struct_newtypes() {
    let mut ascii = Ascii(vec![0; 8]);
    ascii.0[0] = 8;
    ascii.0[1] = 7;
    ascii.0[2] = 6;
    ascii.0[3] = 5;
    ascii.0[4] = 4;
    ascii.0[5] = 3;
    ascii.0[6] = 2;
    ascii.0[7] = 1;
    ascii.0.reverse();

    assert_eq!((1..=8).collect::<Vec<u8>>(), ascii.0);
}

trait Such {
    fn has_value(&self) -> bool;
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
struct Onesuch;

impl Such for Onesuch {
    fn has_value(&self) -> bool {
        false
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
struct Anysuch<T>(T);

impl<T> Such for Anysuch<T> {
    fn has_value(&self) -> bool {
        true
    }
}

#[allow(dead_code)]
fn assert<'a>(expected: bool, s: &(dyn Such + 'a)) {
    assert_eq!(expected, s.has_value());
}

#[test]
fn unit_struct() {
    let o = Onesuch;
    assert(false, &o);
}

#[test]
fn unit_struct_anysuch() {
    let o = Anysuch(8u32);
    assert(true, &o);
}
