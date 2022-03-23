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
}
