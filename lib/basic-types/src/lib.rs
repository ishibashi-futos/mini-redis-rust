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

pub mod reference_struct {
    // このコードはコンパイルできない
    // pub struct S {
    //     r: &i32, // rの生存期間がわからない
    // }

    // &'staticでも良いが、制約がきつすぎる
    // 構造体が生きている間は変数が残っていてほしいので、&'aにする
    pub struct S<'a> {
        pub r: &'a i32,
    }

    pub struct S2 {
        pub r: &'static i32,
    }

    pub struct S3<'a> {
        pub x: &'a i32,
        pub y: &'a i32,
    }

    pub struct S4<'a, 'b> {
        pub x: &'a i32,
        pub y: &'b i32,
    }
}

pub mod field_and_properties {
    pub struct Game {
        pub black_spawns: bool,
    }
}
