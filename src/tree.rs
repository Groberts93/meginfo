use crate::tag::{Data, Tag};

#[derive(Debug, PartialEq, Clone)]
pub struct Tree {
    nodes: Option<Vec<Tree>>, // can we just get rid of the option?
    data: Data,
}

impl Tree {
    pub fn new() -> Self {
        Tree {
            nodes: None,
            data: Data::Void,
        }
    }

    pub fn with_data(data: Data) -> Self {
        Tree { nodes: None, data }
    }

    pub fn add_node(&mut self, tree: Tree) {
        match &mut self.nodes {
            Some(nodes) => nodes.push(tree),
            None => self.nodes = Some(vec![tree]),
        }
    }

    pub fn count_children(tree: &Tree) -> usize {
        tree.nodes.as_ref().map_or(0, |t| t.len())
    }

    pub fn count_nodes(tree: &Tree) -> usize {
        let count = match tree.nodes.as_ref() {
            Some(trees) => trees.iter().fold(1, |c, t| c + Self::count_nodes(t)),
            None => 1,
        };

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_root() {
        assert_eq!(
            Tree::new(),
            Tree {
                nodes: None,
                data: Data::Void
            }
        );
    }

    #[test]
    fn can_create_root_with_node() {
        let mut root = Tree::new();
        let leaf = Tree::new();

        root.nodes = Some(vec![leaf.clone()]);

        assert_eq!(root.nodes, Some(vec![leaf.clone()]));
    }

    #[test]
    fn can_add_node() {
        let mut root = Tree::new();
        let node = Tree::with_data(Data::Float(vec![0.5]));

        root.add_node(node.clone());

        assert_eq!(
            root,
            Tree {
                nodes: Some(vec![Tree {
                    nodes: None,
                    data: Data::Float(vec![0.5])
                }]),
                data: Data::Void
            }
        );
    }

    #[test]
    fn can_count_child_nodes() {
        let mut root = Tree::new();

        for _ in 0..4 {
            root.add_node(Tree::with_data(Data::Void));
        }

        let child_node = &mut root.nodes.as_mut().unwrap()[0];

        for _ in 0..5 {
            child_node.add_node(Tree::new());
        }

        assert_eq!(Tree::count_children(&child_node), 5);
        assert_eq!(Tree::count_children(&root), 4);
    }

    #[test]
    fn can_count_tree_nodes() {
        let mut root = Tree::new();
        let mut count = 1;
        assert_eq!(Tree::count_nodes(&root), count);

        for _ in 0..3 {
            root.add_node(Tree::new());
            count += 1;
        }
        assert_eq!(Tree::count_nodes(&root), count);

        let child_node = &mut root.nodes.as_mut().unwrap()[0];

        for _ in 0..7 {
            child_node.add_node(Tree::new());
            count += 1;
        }
        assert_eq!(Tree::count_nodes(&root), count);

        let child_node_second = &mut root.nodes.as_mut().unwrap()[0].nodes.as_mut().unwrap()[0];

        for _ in 0..2 {
            child_node_second.add_node(Tree::new());
            count += 1;
        }

        assert_eq!(Tree::count_nodes(&root), count);
    }
}
