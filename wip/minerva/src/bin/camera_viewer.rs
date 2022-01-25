//!

use edge::{Renderer, Renderable, Scene};

struct Camera { }
impl Renderable for Camera { }

struct Space { }
impl Renderable for Space { }

fn main() {
    let scene = Scene::new();
    scene.add_renderable("camera", Camera {});
    scene.add_renderable("space", Space {});
    println!("{}", scene);

    let renderer = Renderer::new();
    renderer.draw(scene);
}