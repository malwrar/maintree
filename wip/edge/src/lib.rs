struct AssetManager;
struct UniformManager;
struct Camera;

trait Drawable {
    fn setup(&mut self, engine: &mut Engine) {

    }

    fn render(&self, pass: &mut RenderPass, engine: &Engine) {

    }
}

struct Engine {
    assets: AssetManager,
    vertices: VertexManager,
    pipelines: PipelineManager,
    uniforms: UniformManager,
}

fn main() {
    let window = winit stuff;

    let object = Blah();

    let engine = Engine::new()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}