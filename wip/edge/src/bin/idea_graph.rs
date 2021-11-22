use std::fmt;

use petgraph::graph::{Graph, NodeIndex};
use petgraph::dot::{Dot, Config};

struct RenderGraph {
    graph: Graph::<&'static str, &'static str>,
}

impl RenderGraph {
    fn new() -> Self {
        Self {
            graph: Graph::new(),
        }
    }

    fn add_stage(&mut self, name: &'static str) -> NodeIndex {
        self.graph.add_node(name)
    }

    fn add_step(&mut self, stage: NodeIndex, name: &'static str) {
        let node = self.graph.add_node(name);
        self.graph.add_edge(stage, node, "weight");
    }

}

impl fmt::Display for RenderGraph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}",
               Dot::with_config(
                   &self.graph,
                   &[
                        Config::EdgeNoLabel
                   ]))
    }
}

trait Draw {
    fn draw(&self, render_graph: &mut RenderGraph);
}

struct Triangle { }

impl Draw for Triangle {
    fn draw(&self, render_graph: &mut RenderGraph) {
        let stage = render_graph.add_stage("triangle_bundle");
        render_graph.add_step(stage, "mesh");
        render_graph.add_step(stage, "texture (optional)");
        render_graph.add_step(stage, "named params(s) (position, orientation, etc)");
    }
}

fn main() {
    let mut graph = RenderGraph::new();

    let triangle = Triangle {};
    triangle.draw(&mut graph);
    triangle.draw(&mut graph);
    triangle.draw(&mut graph);

    println!("{}", graph);
}
