extern crate kdtree;

use petgraph::DiGraphMap;

pub struct Renderer {
    pub graph: DiGraphMap,
}

impl Renderer {
    pub fn add_renderable<R>(&mut self, renderable: R) {

    }

    pub fn add_instance<K, R>(&mut self, key: K, renderable: R)
}