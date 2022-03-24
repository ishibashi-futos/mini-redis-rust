#[derive(Debug, PartialEq)]
pub enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

impl<T> BinaryTree<T> {
    pub fn new(e: TreeNode<T>) -> BinaryTree<T> {
        BinaryTree::NonEmpty(Box::new(e))
    }

    pub fn as_ref(&self) -> Option<&TreeNode<T>> {
        match self {
            Self::Empty => None,
            Self::NonEmpty(tree) => Some(tree),
        }
    }

    pub fn as_ref_mut(&mut self) -> Option<&mut TreeNode<T>> {
        match self {
            Self::Empty => None,
            Self::NonEmpty(tree) => Some(tree),
        }
    }
}

impl<T: Ord> BinaryTree<T> {
    pub fn add(&mut self, value: T) {
        match *self {
            BinaryTree::Empty => {
                *self = BinaryTree::new(TreeNode {
                    element: value,
                    left: BinaryTree::Empty,
                    right: BinaryTree::Empty,
                })
            }
            BinaryTree::NonEmpty(ref mut node) => {
                if value <= node.element {
                    node.left.add(value);
                } else {
                    node.right.add(value);
                }
            }
        }
    }
}

impl<T: Eq + Ord> BinaryTree<T> {
    pub fn search(&self, value: T) -> Option<&TreeNode<T>> {
        match self {
            BinaryTree::Empty => None,
            BinaryTree::NonEmpty(node) => {
                if value == node.element {
                    self.as_ref()
                } else if node.element < value {
                    node.as_ref().left.search(value)
                } else {
                    node.as_ref().right.search(value)
                }
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct TreeNode<T> {
    pub element: T,
    pub left: BinaryTree<T>,
    pub right: BinaryTree<T>,
}

#[cfg(test)]
mod tests {

    use super::BinaryTree::{self, *};
    use super::TreeNode;

    #[test]
    fn it_work() {
        let jupiter_tree = BinaryTree::new(TreeNode {
            element: "Jupiter",
            left: Empty,
            right: Empty,
        });

        let mars_tree = BinaryTree::new(TreeNode {
            element: "Mars",
            left: jupiter_tree,
            right: Empty,
        });

        let tree = mars_tree.as_ref();
        assert_eq!("Mars", tree.unwrap().element);
    }

    #[test]
    fn as_ref_mut() {
        let mut jupiter_tree = BinaryTree::new(TreeNode {
            element: "Jupiter",
            left: Empty,
            right: Empty,
        });

        {
            let tree = jupiter_tree.as_ref_mut().unwrap();
            tree.element = "modified";
        }

        assert_eq!(
            BinaryTree::new(TreeNode {
                element: "modified",
                left: Empty,
                right: Empty,
            }),
            jupiter_tree
        );
    }

    #[test]
    fn add() {
        let mut tree = BinaryTree::Empty;
        tree.add(2);
        tree.add(1);
        tree.add(3);

        assert_eq!(
            BinaryTree::new(TreeNode {
                element: 2,
                left: BinaryTree::new(TreeNode {
                    element: 1,
                    left: Empty,
                    right: Empty,
                }),
                right: BinaryTree::new(TreeNode {
                    element: 3,
                    left: Empty,
                    right: Empty,
                })
            }),
            tree
        );
    }

    #[test]
    fn add_and_ref_mut() {
        let mut tree = BinaryTree::Empty;
        tree.add(2);
        tree.add(1);
        tree.add(3);

        let two = tree.as_ref_mut().unwrap();
        let one = two.left.as_ref_mut().unwrap();
        one.element = 10;

        assert_eq!(
            BinaryTree::new(TreeNode {
                element: 2,
                left: BinaryTree::new(TreeNode {
                    element: 10,
                    left: Empty,
                    right: Empty,
                }),
                right: BinaryTree::new(TreeNode {
                    element: 3,
                    left: Empty,
                    right: Empty,
                })
            }),
            tree
        );
    }

    #[test]
    fn search() {
        let tree = BinaryTree::new(TreeNode {
            element: 2,
            left: BinaryTree::new(TreeNode {
                element: 10,
                left: Empty,
                right: Empty,
            }),
            right: BinaryTree::new(TreeNode {
                element: 3,
                left: Empty,
                right: Empty,
            }),
        });

        let actual = tree.search(10).unwrap();

        assert_eq!(10, actual.element);
    }

    #[test]
    fn not_found() {
        let tree = BinaryTree::new(TreeNode {
            element: 2,
            left: BinaryTree::new(TreeNode {
                element: 10,
                left: Empty,
                right: Empty,
            }),
            right: BinaryTree::new(TreeNode {
                element: 3,
                left: Empty,
                right: Empty,
            }),
        });

        let actual = tree.search(255);

        assert_eq!(None, actual);
    }

    #[test]
    fn nested_tree() {
        let tree = BinaryTree::new(TreeNode {
            element: 100,
            left: BinaryTree::new(TreeNode {
                element: 99,
                left: Empty,
                right: Empty,
            }),
            right: BinaryTree::new(TreeNode {
                element: 150,
                left: BinaryTree::new(TreeNode {
                    element: 101,
                    left: Empty,
                    right: Empty,
                }),
                right: BinaryTree::new(TreeNode {
                    element: 160,
                    left: Empty,
                    right: Empty,
                }),
            }),
        });

        let actual = tree.search(160);

        assert_eq!(None, actual);
    }
}
