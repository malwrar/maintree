#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings

// NOTE: Bindings must come before functions that use them!
#import bevy_pbr::mesh_functions

struct FloorMaterial {
    color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> material: FloorMaterial;

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) blend_color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) blend_color: vec4<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = mesh_position_local_to_clip(mesh.model, vec4<f32>(vertex.position, 1.0));
    out.blend_color = vertex.blend_color;
    return out;
}

@fragment
fn fragment(
    @builtin(position) frag_coord: vec4<f32>,  
    #import bevy_pbr::mesh_vertex_output
) -> @location(0) vec4<f32> {
    //return vec4<f32>(0.2, 0.2, 0.2, 1.0);
    var p = frag_coord.xz / frag_coord.w;
    var g = 0.5 * abs(fract(p) - 0.5) / fwidth(p);
    var a = min(min(g.x, g.y), 1.0);
    return vec4<f32>(vec3<f32>(a), 1.0 - a);
}