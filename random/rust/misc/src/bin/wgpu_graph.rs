use petgraph::graphmap::DiGraphMap;
use petgraph::dot::Dot;

fn main() {
    let graph = DiGraphMap::<&str, &str>::from_edges(&[
        // Main
        ("wgpu::ShaderModuleDescriptor",   "wgpu::ShaderModule",   "wgpu::Device().create_shader_module()"),
        ("wgpu::PipelineLayoutDescriptor", "wgpu::PipelineLayout", "wgpu::Device().create_pipeline_layout()"),
        ("wgpu::RenderPipelineDescriptor", "wgpu::RenderPipeline", "wgpu::Device().create_render_pipeline()"),

        // Pipeline Composition
        ("wgpu::PipelineLayout",    "wgpu::RenderPipelineDescriptor", ".layout"),
        ("wgpu::VertexState",       "wgpu::RenderPipelineDescriptor", ".vertex"),
        ("wgpu::FragmentState",     "wgpu::RenderPipelineDescriptor", ".fragment"),
        ("wgpu::PrimitiveState",    "wgpu::RenderPipelineDescriptor", ".primitive"),
        ("wgpu::DepthStencilState", "wgpu::RenderPipelineDescriptor", ".depth_stencil"),
        ("wgpu::MultiSampleState",  "wgpu::RenderPipelineDescriptor", ".multisample"),

        // Set up vertex buffer layout
        ("size_of::<Vertex>",            "wgpu::VertexBufferLayout", ".array_stride"),
        ("wgpu::VertexStepMode::Vertex", "wgpu::VertexBufferLayout", ".step_mode"),
        ("wgpu::VertexAttribute",        "wgpu::VertexBufferLayout", ".attributes"),

        // Set up color target
        ("surface.native_config.format",    "wgpu::ColorTargetState", ".format"),
        ("Some(wgpu::BlendState::REPLACE)", "wgpu::ColorTargetState", ".blend"),
        ("wgpu::ColorWrites::ALL",          "wgpu::ColorTargetState", ".write_mask"),

        // Composition Part Assembly
        ("wgpu::ShaderModule",       "wgpu::VertexState",   ".module"),
        ("entry_point",              "wgpu::VertexState",   ".entry_point"),
        ("wgpu::VertexBufferLayout", "wgpu::VertexState",   ".buffers"),

        ("wgpu::ShaderModule",     "wgpu::FragmentState", ".module"),
        ("entry_point",            "wgpu::FragmentState", ".entry_point"),
        ("wgpu::ColorTargetState", "wgpu::FragmentState", ".targets"),

        ("wgpu::Surface",        "wgpu::SurfaceTexture", "wgpu::Surface().get_current_texture()"),
        ("wgpu::SurfaceTexture", "wgpu::TextureView", "wgpu::SurfaceTexture().texture.create_view()"),
        ("wgpu::Device",         "wgpu::CommandEncoder", "wgpu::Device().device.create_command_encoder()"),
        ("wgpu::CommandEncoder", "wgpu::RenderPass", "wgpu::CommandEncoder().begin_render_pass()"),
        ("wgpu::RenderPassDescriptor", "wgpu::RenderPass", "wgpu::CommandEncoder().begin_render_pass()"),

        // Render pass
        ("wgpu::RenderPassDepthStencilAttachment", "wgpu::RenderPassDescriptor", ".depth_stencil_attachment"),
        ("depth_texture.view", "wgpu::RenderPassDepthStencilAttachment", ".view"),
        ("wgpu::Operations",   "wgpu::RenderPassDepthStencilAttachment", ".depth_ops"),

        ("wgpu::RenderPassColorAttachment",        "wgpu::RenderPassDescriptor", ".color_attachments"),
        ("wgpu::TextureView", "wgpu::RenderPassColorAttachment", ".view"),
        ("wgpu::Operations",  "wgpu::RenderPassColorAttachment", ".ops"),

        //
        ("wgpu::RenderPass",     "render_pass.set_vertex_buffer()", ""),
        ("wgpu::RenderPass",     "render_pass.set_index_buffer()", ""),
        ("wgpu::RenderPass",     "render_pass.set_pipeline()", ""),
        ("wgpu::RenderPass",     "render_pass.set_bind_group()", ""),
        ("wgpu::RenderPass",     "render_pass.draw_model_instances()", ""),
        ("wgpu::RenderPass", "nu", "render_pass.draw_indexed()"),
    ]);

    let depth_texture = DiGraphMap::<&str, &str>::from_edges(&[
        // Extent
        ("width: u32", "size: wgpu::Extent3d", ""),
        ("height: u32", "size: wgpu::Extent3d", ""),

        // TextureDescriptor
        ("size: wgpu::Extent3d", "desc: wgpu::TextureDescriptor", ""),

        // Texture
        ("device: &wgpu::Device",         "texture: wgpu::Texture", ""),
        ("desc: wgpu::TextureDescriptor", "texture: wgpu::Texture", ""),

        // TextureView
        ("texture: wgpu::Texture", "view: wgpu::TextureView", ""),

        // Sampler
        ("device: &wgpu::Device", "sampler: wgpu::Sampler", ""),
        ("device: &wgpu::Device", "&wgpu::SampleDescriptor", ""),
    ]);

    let surface = DiGraphMap::<&str, &str>::from_edges(&[
        // Surface
        ("wgpu::Instance", "wgpu::Surface", ""),

        // Adapter
        ("wgpu::Surface",  "wgpu::Adapter", ""),
        ("wgpu::Instance", "wgpu::Adapter", ""),

        // Device, Queue
        ("wgpu::Adapter", "wgpu::Device", ""),
        ("wgpu::Adapter", "wgpu::Queue", ""),

        // SurfaceConfiguration
        ("wgpu::Surface", "surface.get_preferred_format(&adapter)", "surface"),
        ("wgpu::Adapter", "surface.get_preferred_format(&adapter)", "adapter"),

        ("surface.get_preferred_format(&adapter)", "wgpu::SurfaceConfiguration", "format"),
        ("width: u32",                             "wgpu::SurfaceConfiguration", "width"),
        ("height: u32",                            "wgpu::SurfaceConfiguration", "width"),

        ("wgpu::Surface",              "surface.configure(&device, &config)", "surface"),
        ("wgpu::Device",               "surface.configure(&device, &config)", "device"),
        ("wgpu::SurfaceConfiguration", "surface.configure(&device, &config)", "config"),

        // Texture BindGroupLayout
        ("wgpu::BindGroupLayoutEntry { \
            binding: 0, \
            visibility: wgpu::ShaderStages::FRAGMENT, \
            ty: wgpu::BindingType::Texture { \
                multisampled: false,
                view_dimension: wgpu::TextureViewDimension::D2,
                sample_type: wgpu::TextureSampleType::Float { filterable: true },
            }, \
            count: None, \
        }", "entries[]", ""),

        ("wgpu::BindGroupLayoutEntry { \
            binding: 1, \
            visibility: wgpu::ShaderStages::FRAGMENT, \
            ty: wgpu::BindingType::Sampler(wgpu::SampleBindingType::Filtering), \
            count: None, \
        }", "entries[]", ""),

        ("entries[]", "wgpu::BindGroupLayoutDescriptor", ""),

        ("wgpu::BindGroupLayoutDescriptor", "device.create_bind_group_layout(&desc)", ""),
        ("wgpu::Device",                    "device.create_bind_group_layout(&desc)", ""),

        ("device.create_bind_group_layout(&desc)", "texture_bind_group: wgpu::BindGroupLayout", ""),

        // Camera BindGroupLayout
        ("width: u32",  "Camera", ""),
        ("height: u32", "Camera", ""),
    ]);

    let highlevel = DiGraphMap::<&str, &str>::from_edges(&[
        ("uniform", "", ""),
    ]);

    println!("{}", Dot::new(&highlevel));
}