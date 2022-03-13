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
    pub child: Child
}

// 子にもCopyをつける
#[derive(Clone, Copy, Debug)]
pub struct Child {
    pub id: u32,
}
