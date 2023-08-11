use petgraph::dot::{Config, Dot};
use petgraph::Graph;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_make_graph() {
        let graph = make_dummy_graph();

        assert_eq!(graph.node_count(), 3);
        assert_eq!(graph.edge_count(), 2);
        assert_eq!(graph.node_weights().fold(0, |acc, x| acc + x), 6);
    }

    #[test]
    fn can_write_dot() {
        let graph = make_dummy_graph();

        let dot = Dot::new(&graph);

        println!("{dot:?}");
    }

    fn make_dummy_graph() -> Graph<i32, f64> {
        let mut graph = Graph::new();

        let n1 = graph.add_node(2);
        let n2 = graph.add_node(3);
        let n3 = graph.add_node(1);

        let _e1 = graph.add_edge(n1, n2, 0.5);
        let _e2 = graph.add_edge(n2, n3, 0.7);

        graph
    }
}
