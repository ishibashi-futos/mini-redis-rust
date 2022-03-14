pub struct Person {
    pub name: String,
    pub birth: u32,
}

pub struct Person2 {
    pub name: Option<String>,
    pub birth: u32,
}

// struct, enumはデフォルトではCopy Typeではない
// Copy, Cloneトレイトを追加することで、Copy Typeになる
#[derive(Copy, Clone)]
pub struct Label {
    pub number: u32,
}

// 親だけにClone, Copyをつけてもダメ
#[derive(Clone, Copy, Debug)]
pub struct Parent {
    pub child: Child,
}

// 子にもCopyをつける
#[derive(Clone, Copy, Debug)]
pub struct Child {
    pub id: u32,
}

pub mod reference {
    use std::collections::HashMap;

    pub type Table = HashMap<String, Vec<String>>;
    pub fn show(table: &Table) {
        for (artist, works) in table {
            println!("works by {}: ", artist);
            for work in works {
                println!("  {}", work);
            }
        }
    }

    pub fn sort_works(table: &mut Table) {
        for (_artist, works) in table {
            works.sort();
        }
    }

    pub struct Anime {
        pub name: &'static str,
        pub bechdel_pass: bool,
    }

    #[derive(Debug, PartialEq)]
    pub struct Point {
        pub x: i32,
        pub y: i32,
    }

    impl Point {
        pub fn new(x: i32, y: i32) -> Point {
            Point { x, y }
        }
    }
}
