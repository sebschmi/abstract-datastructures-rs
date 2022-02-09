#![warn(missing_docs)]
//! This crate offers functions to read and write graphs in TSPLIB format.

use std::io::Write;
use traitgraph::index::GraphIndex;
use traitgraph::interface::StaticGraph;

/// Write the graph in the following format, ignoring node and edge data.
///
/// ```text
/// <node count> <edge count>
/// <from node> <to node>
/// ```
///
/// The second line is repeated for each edge.
pub fn write_topology<Graph: StaticGraph, Writer: Write>(graph: &Graph, writer: &mut Writer) {
    writeln!(writer, "{} {}", graph.node_count(), graph.edge_count()).unwrap();
    for node in graph.node_indices() {
        for out_neighbor in graph.out_neighbors(node) {
            writeln!(
                writer,
                "{} {}",
                node.as_usize(),
                out_neighbor.node_id.as_usize()
            )
            .unwrap();
        }
    }
}
