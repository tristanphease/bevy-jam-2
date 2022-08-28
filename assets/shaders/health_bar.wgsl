#import bevy_sprite::mesh2d_view_bindings
#import bevy_sprite::mesh2d_bindings

// NOTE: Bindings must come before functions that use them!
#import bevy_sprite::mesh2d_functions

struct Vertex {
    @location(0) position: vec3<f32>, //12 bytes
    @location(1) normal: vec3<f32>, //12 bytes
    @location(2) uv: vec2<f32>, //8 bytes
};

struct HealthBarMaterial {
    color: vec4<f32>,
    amount: f32,
    width: f32,
}

@group(1) @binding(0)
var<uniform> material: HealthBarMaterial;

struct VertexOutput {
    @location(0) uv: vec2<f32>, //8 bytes
    @location(1) vert_pos: vec2<f32>, //8 bytes
    @builtin(position) clip_position: vec4<f32>, //16 bytes
};

@vertex 
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    out.uv = vertex.uv;
    out.clip_position = mesh2d_position_local_to_clip(mesh.model, vec4<f32>(vertex.position, 1.0));
    out.vert_pos = vertex.position.xy;
    return out;
}

@fragment
fn fragment(
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) vert_pos: vec2<f32>,
) -> @location(0) vec4<f32> {
    if (uv.x < 1.0) {
        //inside health bar
        if (vert_pos.x < (material.amount * material.width - material.width/2.0)) {
            //health full up to here
            return material.color;
        }
        //health not full here
        return vec4<f32>(1.0, 1.0, 1.0, 1.0);
    }
    //border
    return vec4<f32>(0.0, 0.0, 0.0, 1.0);
}