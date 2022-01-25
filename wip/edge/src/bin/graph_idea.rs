//! Idea where scenes and render stages are represented as graphs, and worked
//! on concurrently by the renderer. Scenegraphs & rendergraphs/framegraphs
//! aren't altogether that new, the goal of this one is to take advantage of
//! rust language features to make it painless to use while still getting you
//! fairly close to wgpu.

// Something that can be rendered. Must provide geometry

extern crate petgraph;

use std::collections::HashMap;

use petgraph::{Graph, NodeIndex};

trait Renderable {}

struct Triangle {}
impl Renderable for Triangle {}

#[derive(Debug)]
enum SceneNode<K, R> where R: Renderable {
    World,
    Group(K),
    Renderable(K, R),
}

#[derive(Debug)]
struct InstanceProperties {

}

// A collection of objects in 3d space, designed to be efficiently rendered
// while also being easy to update.
struct Scene<K, R> {
    graph: Graph<SceneNode<R>, InstanceProperties>
    world: NodeIndex<SceneNode::World>,
    groups: HashMap<K, NodeIndex<SceneNode::Renderable<R>>>,
    renderables: HashMap<K, NodeIndex<SceneNode::Renderable<R>>>,
}

impl<K, R> Scene<K, R> {
    fn new() -> Self {
        let graph = Graph::new();
        let world = graph.add_node(SceneNode::World);

        Self {
            graph,
            world,
            groups: HashMap::new(),
            renderables: HashMap::new(),
        }
    }
}

fn main() {
    let scene = Scene::new();

    let triangle = Triangle::new(); 
    scene.add_renderable("", triangle);

    println!("{:?}", scene);
}