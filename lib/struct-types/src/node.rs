use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct Node {
    pub name: String,
    pub children: Vec<Rc<Self>>,
}

impl Node {
    pub fn new(name: String, children: Vec<Rc<Node>>) -> Node {
        Node { name, children }
    }

    // あるメソッドがSelfへのポインタの所有権を取得しなければならない場合、selfのパラメータの型を明示的に書く必要がある
    pub fn append_to(self: Rc<Self>, parent: &mut Node) {
        parent.children.push(self);
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::Node;
    #[test]
    fn it_work() {
        let child = Node::new("child".to_string(), vec![]);
        let child = Rc::new(child);
        let mut parent = Node::new("parent".to_string(), vec![]);

        child.append_to(&mut parent);
        // Node::append_to(child, &mut parent); // こんなふうにも書ける

        assert_eq!(
            Node::new(
                "parent".to_string(),
                vec![Rc::new(Node::new("child".to_string(), vec![]))]
            ),
            parent
        );
    }
}
