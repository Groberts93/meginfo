use crate::tag::{Data, Tag};

#[derive(Debug, PartialEq, Clone)]
pub struct Tree {
    nodes: Vec<Tree>,
    data: Data,
}

impl Tree {
    pub fn new() -> Self {
        Tree {
            nodes: vec![],
            data: Data::Void,
        }
    }

    pub fn with_data(data: Data) -> Self {
        Tree {
            nodes: vec![],
            data,
        }
    }

    pub fn add_node(&mut self, tree: Tree) {
        self.nodes.push(tree);
    }

    pub fn count_nodes(tree: &Tree) -> usize {
        match tree.nodes.len() {
            0 => 1,
            _ => tree.nodes.iter().fold(1, |c, t| c + Self::count_nodes(t)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_root() {
        // create a root node with no data
        assert_eq!(
            Tree::new(),
            Tree {
                nodes: vec![],
                data: Data::Void
            }
        );
    }

    #[test]
    fn can_add_node() {
        // create a root node, then add another node as a child
        let mut root = Tree::new();
        let node = Tree::with_data(Data::Float(vec![0.5]));

        root.add_node(node.clone());

        assert_eq!(
            root,
            Tree {
                nodes: vec![Tree {
                    nodes: vec![],
                    data: Data::Float(vec![0.5])
                }],
                data: Data::Void
            }
        );
    }

    #[test]
    fn can_count_tree_nodes() {
        // create a new tree, add nodes, and check the count is correct
        let mut root = Tree::new();
        let mut count = 1;  // the root node is the only node in the tree
        assert_eq!(Tree::count_nodes(&root), count);

        for _ in 0..3 {
            root.add_node(Tree::new());
            count += 1;
        }
        assert_eq!(Tree::count_nodes(&root), count);

        let child_node = &mut root.nodes[0];

        for _ in 0..7 {
            child_node.add_node(Tree::new());
            count += 1;
        }
        assert_eq!(Tree::count_nodes(&root), count);

        let child_node_second = &mut root.nodes[0].nodes[0];

        for _ in 0..2 {
            child_node_second.add_node(Tree::new());
            count += 1;
        }

        assert_eq!(Tree::count_nodes(&root), count);
    }
}
