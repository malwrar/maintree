// Draws a grid on an infinite plane.
//
// Math based on: https://asliceofrendering.com/scene%20helper/2020/01/05/InfiniteGrid/

#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings
#import bevy_pbr::mesh_functions

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,

    // outputs that bevy's pbr expects. Note that in our fork this ends at
    // @location(4).
    //
    // [1]: https://github.com/malwrar/bevy/blob/release-0.8.1/crates/bevy_pbr/src/render/mesh_vertex_output.wgsl#L12
    #import bevy_pbr::mesh_vertex_output

    @location(5) near_position: vec3<f32>,
    @location(6) far_position: vec3<f32>,
};

fn unproject_point(pt: vec3<f32>) -> vec3<f32> {
    let pt = vec4<f32>(pt, 1.0);  // homogenous
    let pt = view.inverse_view * view.inverse_projection * pt;
    let pt = vec3<f32>(pt.xyz / pt.w);  // euclidian

    return pt;
}

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    var model = mesh.model;

    out.world_normal = mesh_normal_local_to_world(vertex.normal);
    out.world_position = mesh_position_local_to_world(model, vec4<f32>(vertex.position, 1.0));
    out.clip_position = mesh_position_world_to_clip(out.world_position);
    out.near_position = unproject_point(vec3<f32>(vertex.position.xy, 0.0));
    out.far_position = unproject_point(vec3<f32>(vertex.position.xy, 1.0));

    return out;
}

struct FragmentInput {
    #import bevy_pbr::mesh_vertex_output
    @location(5) near_position: vec3<f32>,
    @location(6) far_position: vec3<f32>,
};

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    let color = vec3<f32>(1.0, 0.0, 0.0);

    return vec4<f32>(color, 1.0);
}