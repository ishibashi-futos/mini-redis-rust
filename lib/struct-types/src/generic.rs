pub struct Queue<T> {
    older: Vec<T>,
    younger: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            older: Vec::new(),
            younger: Vec::new(),
        }
    }

    pub fn push(&mut self, t: T) {
        self.younger.push(t);
    }

    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }
            std::mem::swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }
        self.older.pop()
    }

    pub fn split(self) -> (Vec<T>, Vec<T>) {
        (self.older, self.younger)
    }
}

impl Queue<f64> {
    /// sum all elements
    /// ```
    /// let mut q = struct_types::generic::Queue::<f64>::new();
    /// q.push(1.0);
    /// q.push(2.0);
    /// assert_eq!(3.0, q.sum());
    /// ```
    pub fn sum(&self) -> f64 {
        self.older.iter().sum::<f64>() + self.younger.iter().sum::<f64>()
    }
}

#[test]
fn pop() {
    let mut q = Queue::<f64>::new();
    q.push(1.0);
    q.push(3.0);
    q.pop();
    assert_eq!(3.0, q.sum());
}
