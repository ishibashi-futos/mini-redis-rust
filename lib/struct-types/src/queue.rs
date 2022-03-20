/// charの先入先出キュー
pub struct Queue {
    older: Vec<char>,
    younger: Vec<char>,
}

impl Queue {
    pub fn new() -> Queue {
        Queue {
            older: Vec::new(),
            younger: Vec::new(),
        }
    }

    /// キューにアイテムをpushする
    ///
    /// ```
    /// let mut queue = struct_types::queue::Queue::new();
    /// queue.push('a');
    /// ```
    pub fn push(&mut self, c: char) {
        self.younger.push(c);
    }

    /// キューの先端から文字をポップする
    /// Popする文字があれば`Some(char)`を返し、空であれば`None`を返す
    ///
    /// ```
    /// let mut queue = struct_types::queue::Queue::new();
    /// queue.push('0');
    /// queue.push('1');
    /// assert_eq!(Some('0'), queue.pop());
    /// ```
    ///
    /// ```
    /// let mut queue = struct_types::queue::Queue::new();
    /// assert_eq!(None, queue.pop());
    /// ```
    pub fn pop(&mut self) -> Option<char> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }
            std::mem::swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }
        self.older.pop()
    }

    /// 借用で十分であれば`&self`を使う
    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }

    /// 所有権を取得Sたければ`self`で受け取ることもできる
    pub fn split(self) -> (Vec<char>, Vec<char>) {
        (self.older, self.younger)
    }

    /// 不必要に`self`にしてしまうと所有権が移ってしまうので注意
    pub fn is_empty_owned(self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }
}
