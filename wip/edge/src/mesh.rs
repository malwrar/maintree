//!

pub struct MeshBuilder<T> 
where
    T: Copy,
{
    vertices: Vec<Vertex<T>>,
    indices: Vec<usize>,
}

impl<T> MeshBuilder<T>
where
    T: Copy,
{
    fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices:  Vec::new(),
        }
    }

    fn vertices(&mut self, vertices: &[(f32, f32, f32, T)]) {
        for vertex in vertices {
            self.vertices.push(Vertex {
                x: vertex.0,
                y: vertex.1,
                z: vertex.2,
                attrs: vertex.3,
            });
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vertex<T>
where
    T: Copy,
{
    x: f32,
    y: f32,
    z: f32,
    attrs: T
}

pub struct Mesh<T>
where
    T: Copy,
{
    vertices: Vec<Vertex<T>>,
    indices: Vec<usize>,
}

impl<T> Mesh<T>
where
    T: Copy,
{
    pub fn new() -> MeshBuilder<T> {
        MeshBuilder::new()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}