//!

extern crate nalgebra;
extern crate wgpu;
extern crate winit;

pub mod mesh;
pub use crate::mesh::{Mesh, MeshBuilder};

pub mod renderer;
pub use crate::renderer::{Renderer, Renderable};

pub mod shader;
pub mod buffer;
pub mod pipeline;

//use std::fmt;
//use std::collections::hash_map::HashMap;
//
//extern crate petgraph;
//use petgraph::{
//    graph::{DefaultIx, Graph, NodeIndex},
//    dot::Dot,
//};
//
/////
//#[derive(Debug)]
//pub enum SceneNode<K, R>
//where
//    K: fmt::Display + fmt::Debug + PartialEq,
//    R: fmt::Debug + Renderable,
//{
//    World,
//    Group(K),
//    Renderable(K, R),
//}
//
//impl<K, R> fmt::Display for SceneNode<K, R>
//where
//    K: fmt::Display + fmt::Debug + PartialEq,
//    R: fmt::Debug + Renderable,
//{
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        write!(f, "{:?}", self)
//    }
//}
// 
/////
//#[derive(Debug)]
//pub struct Scene<K, R>
//where
//    K: fmt::Display + fmt::Debug + PartialEq,
//    R: fmt::Debug + Renderable,
//{
//    graph: Graph<SceneNode<K, R>, &'static str>,
//    world: NodeIndex<DefaultIx>,
//    renderables: HashMap<K, NodeIndex<DefaultIx>>,
//}
//
//impl<K, R> Scene<K, R>
//where
//    K: fmt::Display + fmt::Debug + PartialEq,
//    R: fmt::Debug + Renderable,
//{
//    fn new() -> Self {
//        let mut graph = Graph::new();
//        let world = graph.add_node(SceneNode::World);
//
//        Self {
//            graph,
//            world,
//            renderables: HashMap::new(),
//        }
//    }
//
//    fn add_group(id: K) {
//
//    }
//
//    fn push_renderable(id: K, renderable: R) {
//
//    }
//
//    //fn pop_renderable(id: K) -> R {
//
//    //}
//
//    fn spawn_instance(child: K, parent: R) {
//        // TODO: properties? id? these are basically entities, how to make them CRUDable?
//    }
//}
//
//impl<K, R> Renderable for Scene<K, R>
//where
//    K: fmt::Display + fmt::Debug + PartialEq,
//    R: fmt::Debug + Renderable,
//{
//
//}
//
//impl<K, R> fmt::Display for Scene<K, R>
//where
//    K: fmt::Display + fmt::Debug + PartialEq,
//    R: fmt::Debug + Renderable,
//{
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        write!(f, "{}", Dot::new(&self.graph))
//    }
//}