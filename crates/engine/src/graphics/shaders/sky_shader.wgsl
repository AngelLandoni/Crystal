[[location(0)]]
var<in> in_position: vec4<f32>;
[[location(1)]]
var<in> in_vertex_color: vec4<f32>;
[[location(2)]]
var<in> in_uv: vec4<f32>;

[[builtin(position)]]
var<out> out_pos: vec4<f32>;
[[location(1)]]
var<out> out_vertex_color: vec4<f32>;

[[location(0)]]
var<out> out_color: vec3<f32>;

[[block]]
struct Locals {
    view: mat4x4<f32>;
    projection: mat4x4<f32>;
};
[[group(0), binding(0)]]
var r_locals: Locals;

[[block]]
struct Sky {
    start_color: vec3<f32>;
    end_color: vec3<f32>;
};
[[group(1), binding(0)]]
var r_sky: Sky;

[[stage(vertex)]]
fn vs_main() {
	out_color = r_sky.start_color;
	
    var no_translation_transform: mat4x4<f32> = r_locals.view;
    no_translation_transform.w.x = 0.0;
    no_translation_transform.w.y = 0.0;
    no_translation_transform.w.z = 0.0;

    // Recreate the transformation matrix.
    out_pos = r_locals.projection * no_translation_transform * in_position;

    out_vertex_color = in_vertex_color;
}

[[location(0)]]
var<out> out_color: vec4<f32>;

[[location(0)]]
var<in> in_color_fs: vec3<f32>;
[[location(1)]]
var<in> fs_in_vertex_color: vec4<f32>;

[[stage(fragment)]]
fn fs_main() {
    out_color = vec4<f32>(fs_in_vertex_color.x, fs_in_vertex_color.y, fs_in_vertex_color.z, 1.0);
}