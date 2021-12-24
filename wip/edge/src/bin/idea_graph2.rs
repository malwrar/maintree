//!

use petgraph::graph::{Graph, NodeIndex};
use petgraph::dot::{Dot, Config};

fn main() {
    let mut graph = Graph::new();

    let world = graph.add_node("world");
    let car = graph.add_node("car");
    let car_chassis = graph.add_node("car_chassis");
    let wheel1 = graph.add_node("wheel1");
    let wheel2 = graph.add_node("wheel2");
    let wheel3 = graph.add_node("wheel3");
    let wheel4 = graph.add_node("wheel4");

    let cost_1 = graph.add_edge(world, car, (0.0, 0.0, 0.0));
    let cost_2 = graph.add_edge(car, car_chassis, (0.0, 0.0, 0.0));
    let cost_3 = graph.add_edge(car_chassis, wheel1, (0.0, 0.0, 0.0));
    let cost_4 = graph.add_edge(car_chassis, wheel2, (0.0, 0.0, 0.0));
    let cost_5 = graph.add_edge(car_chassis, wheel3, (0.0, 0.0, 0.0));
    let cost_6 = graph.add_edge(car_chassis, wheel4, (0.0, 0.0, 0.0));

    println!("{:?}", Dot::new(&graph));
}
