use crate::wgpu;

pub trait VertexBuffer {
    fn layout<'a>(&self) -> wgpu::VertexBufferLayout<'a>;
    fn primitive_state(&self) -> wgpu::PrimitiveState;
    fn upload(&self, device: &wgpu::Device) -> (wgpu::Buffer, Option<wgpu::Buffer>);
}