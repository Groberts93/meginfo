use std::fmt::Display;
use petgraph::stable_graph::{NodeIndex, StableGraph};
use petgraph::Directed;

use termtree;

#[derive(Debug, Clone)]
pub struct Tree<T: Display> {
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

    fn create_view(&self, node: NodeIndex) -> termtree::Tree<String> {
        let mut tree = termtree::Tree::new(self.graph[node].to_string());

        let children: Vec<NodeIndex> = self.graph.neighbors(node).collect();

        for child in children.iter().rev() {
            tree.push(self.create_view(*child));
        }

        tree
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
}

impl<T: Default + PartialEq + Display> Display for Tree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.create_view(self.root))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::dot::Dot;

    #[test]
    fn can_make_tree() {
        let tree = make_dummy_tree();

        assert_eq!(tree.node_count(), 8);
        assert_eq!(tree.edge_count(), 7);
        assert_eq!(tree.graph.node_weights().fold(0, |acc, x| acc + x), 28);
    }

    #[test]
    fn can_write_dot() {
        let tree = make_dummy_tree();
        let _dot = Dot::new(&tree.graph);
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
