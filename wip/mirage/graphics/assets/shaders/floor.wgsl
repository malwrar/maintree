#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings

// NOTE: Bindings must come before functions that use them!
#import bevy_pbr::mesh_functions

struct FloorMaterial {
    color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> material: FloorMaterial;

@fragment
fn fragment(
    #import bevy_pbr::mesh_vertex_output
) -> @location(0) vec4<f32> {
    if (fract(uv.x / 0.1) < 0.01 || fract(uv.y / 0.1) < 0.01) {
        return vec4<f32>(0.0, 0.0, 0.0, 1.0);
    } else {
        return vec4<f32>(0.2, 0.2, 0.2, 1.0);
    }
}