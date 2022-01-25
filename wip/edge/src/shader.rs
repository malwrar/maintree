use crate::buffer::VertexBuffer;

struct Shader<V: VertexBuffer> {
    src: String,
    entry_point: &'static str,
}

impl<T: VertexBuffer> Shader<T> {
    fn new(entry_point: &'static str, src: String) -> Self {
        Self {
            entry_point,
            src,
        }
    }

    fn upload(&self, device: &wgpu::Device) -> wgpu::VertexState {
        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(self.src.into()),
        });

        wgpu::VertexState {
            module: &shader,
            entry_point: self.entry_point.into(),
            buffers: &[
                T::describe(),
            ],
        }
    }
}