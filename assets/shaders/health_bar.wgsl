#import bevy_sprite::mesh2d_view_bindings
#import bevy_sprite::mesh2d_bindings

// NOTE: Bindings must come before functions that use them!
#import bevy_sprite::mesh2d_functions

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

@group(1) @binding(0)
var<uniform> amount: f32;
@group(1) @binding(1)
var<uniform> width: f32;

struct VertexOutput {
    @location(0) uv: vec2<f32>,
    @location(1) x_pos: f32,
    @builtin(position) clip_position: vec4<f32>,
};

@vertex 
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    out.uv = vertex.uv;
    out.clip_position = mesh2d_position_local_to_clip(mesh.model, vec4<f32>(vertex.position, 1.0));
    out.x_pos = vertex.position.x;
    return out;
}

@fragment
fn fragment(
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) x: f32,
) -> @location(0) vec4<f32> {
    if (uv.x < 1.0) {
        //inside health bar
        if (x < (amount * width - width/2.0)) {
            //health full up to here
            return vec4<f32>(1.0, 0.0, 0.0, 1.0);
        }
        //health not full here
        return vec4<f32>(1.0, 1.0, 1.0, 1.0);
    }
    //border
    return vec4<f32>(0.0, 0.0, 0.0, 1.0);
}