use std::fmt::Display;

use crate::tag::{Data, Tag};

#[derive(Debug, PartialEq, Clone)]
pub struct Tree<T> {
    nodes: Vec<Tree<T>>,
    data: T,
}

impl<T: Default + PartialEq + Display> Tree<T> {
    pub fn new() -> Self {
        Tree {
            nodes: vec![],
            data: T::default(),
        }
    }

    pub fn with_data(data: T) -> Self {
        Tree {
            nodes: vec![],
            data,
        }
    }

    pub fn add_node(&mut self, tree: Tree<T>) {
        self.nodes.push(tree);
    }

    pub fn count_nodes(tree: &Tree<T>) -> usize {
        match tree.nodes.len() {
            0 => 1,
            _ => tree.nodes.iter().fold(1, |c, t| c + Self::count_nodes(t)),
        }
    }

    pub fn find_node<'a>(tree: &'a Tree<T>, data: &'a T) -> Option<&'a Tree<T>> {
        if tree.data == *data {
            Some(&tree)
        } else if tree.nodes.len() > 0 {
            tree.nodes.iter().find_map(|t| Self::find_node(t, &data))
        } else {
            None
        }
    }

    fn write_children(depth: usize, is_last_node: bool, tree: &Tree<T>) -> Vec<char> {
        let mut chars = vec![];

        for ii in 0..=depth {
            match depth - ii {
                0 => {
                    for char in tree.data.to_string().chars() {
                        chars.push(char);
                    }
                }
                1 => {
                    match is_last_node {
                        false => chars.push('├'),
                        true => chars.push('└'),
                    }
                    chars.push('─');
                }
                _ => {
                    chars.push('│');
                    chars.push(' ');
                }
            }
        }

        for (ii, node) in tree.nodes.iter().enumerate() {
            chars.push('\n');
            chars.append(&mut Self::write_children(
                depth + 1,
                ii == tree.nodes.len() - 1,
                node,
            ));
        }

        chars
    }
}

impl<T: Default + PartialEq + Display> Display for Tree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let buf = Tree::write_children(0, false, &self);
        let buf: String = buf.iter().collect();
        write!(f, "{}", buf)
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
        let node = Tree::with_data(0.5f32);

        root.add_node(node.clone());

        assert_eq!(
            root,
            Tree {
                nodes: vec![Tree {
                    nodes: vec![],
                    data: 0.5
                }],
                data: 0.0
            }
        );
    }

    #[test]
    fn can_count_tree_nodes() {
        // create a new tree, add nodes, and check the count is correct
        let mut root: Tree<Tag> = Tree::new();
        let mut count = 1; // the root node is the only node in the tree
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

    #[test]
    fn can_find_node() {
        let mut root = Tree::new();

        for ii in 0..3 {
            root.add_node(Tree::with_data(ii));
        }

        for ii in 7..12 {
            root.nodes[0].add_node(Tree::with_data(ii));
        }

        for ii in 0..3 {
            root.nodes[0].nodes[3].add_node(Tree::with_data(ii));
        }

        let target = Tree::find_node(&root, &7).unwrap();

        println!("{root}");
        assert_eq!(target.data, 7);
    }
}
