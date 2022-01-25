//!
extern crate wgpu;

trait Renderable {
    fn desc();
    fn draw(render_pass: &mut wgpu::RenderPass);
}

struct Scene {

}

impl Renderable for Scene {
    fn desc(&self) {

    }

    fn draw(&self, render_pass: &mut wgpu::RenderPass) {

    }
}

fn main() {

    println!("");
}