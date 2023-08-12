use std::collections::HashSet;
use std::fmt::Display;

use petgraph::data::Build;
use petgraph::dot::{Config, Dot};
use petgraph::stable_graph::{EdgeIndex, NodeIndex, StableGraph, StableUnGraph};
use petgraph::{Directed, Graph, Undirected};

#[derive(Debug, Clone, Default)]
pub struct Tree<T> {
    graph: StableGraph<T, ()>,
    current: NodeIndex,
    pub root: NodeIndex,
}

impl<T> Tree<T>
where
    T: Default + Display + PartialEq,
{
    pub fn new() -> Self {
        let mut graph: StableGraph<T, (), Directed, u32> = StableGraph::default();
        let root = graph.add_node(T::default());

        Tree {
            graph: graph,
            current: root,
            root: root,
        }
    }

    pub fn add_child(&mut self, child: T) -> NodeIndex {
        // adds a child at the current node
        let n = self.graph.add_node(child);
        self.graph.add_edge(self.current, n, ());
        n
    }

    pub fn move_to(&mut self, ind: NodeIndex) {
        // sets current to a supplied nodeindex
        self.current = ind;
    }

    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    pub fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }

    fn display_node_at_depth(node: &T, depth: usize) -> Vec<char> {
        let mut chars = vec![];
        chars.push('\n');

        for ii in 0..=depth {
            match depth - ii {
                0 => {
                    for char in node.to_string().chars() {
                        chars.push(char);
                    }
                }
                1 => {
                    // match is_last_node {
                    //     false => chars.push('├'),
                    //     true => chars.push('└'),
                    // }
                    chars.push('├');
                    chars.push('─');
                }
                _ => {
                    chars.push('│');
                    chars.push(' ');
                }
            }
        }
        chars
    }

    fn display(&self, start: NodeIndex) -> Vec<char> {
        let mut curr = start;
        let mut chars: Vec<char> = self.graph[curr].to_string().chars().collect();
        let mut visited: HashSet<NodeIndex> = HashSet::new();
        let mut branch = vec![];

        'outer: loop {
            visited.insert(curr);

            // horrible, but .rev() doesn't work and I need insertion order
            for node in self
                .graph
                .neighbors(curr)
                .collect::<Vec<NodeIndex>>()
                .iter()
                .rev()
            {
                if !visited.contains(&node) {
                    branch.push(curr);
                    curr = *node;

                    chars.append(&mut Tree::display_node_at_depth(
                        &self.graph[curr],
                        branch.len(),
                    ));

                    continue 'outer;
                }
            }

            curr = branch.pop().unwrap();

            if visited.len() == self.graph.node_count() {
                break;
            }
        }

        // for (ii, node) in iter().enumerate() {
        //     chars.push('\n');
        //     chars.append(&mut Self::write_children(
        //         depth + 1,
        //         ii == tree.nodes.len() - 1,
        //         node,
        //     )

        chars
    }
}

impl<T: Default + PartialEq + Display> Display for Tree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let buf = &self.display(self.root);

        let buf: String = buf.iter().collect();
        write!(f, "{}", buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_make_tree() {
        let tree = make_dummy_tree();

        println!("{tree}");
        assert_eq!(tree.node_count(), 8);
        assert_eq!(tree.edge_count(), 7);
        assert_eq!(tree.graph.node_weights().fold(0, |acc, x| acc + x), 28);
    }

    #[test]
    fn can_write_dot() {
        let tree = make_dummy_tree();
        let dot = Dot::new(&tree.graph);

        println!("{dot:?}");
    }

    fn make_dummy_tree() -> Tree<i32> {
        let mut tree = Tree::new();

        let _a = tree.add_child(1);
        let _b = tree.add_child(2);
        let c = tree.add_child(3);

        tree.move_to(c);
        let d = tree.add_child(4);
        let _e = tree.add_child(5);

        tree.move_to(d);
        let _f = tree.add_child(6);
        let _g = tree.add_child(7);

        tree
    }
}
