
[[builtin(vertex_index)]]
var<in> in_vertex_index: u32;

[[builtin(position)]]
var<out> out_pos: vec4<f32>;

// SEND TO FRAGMENT.
[[location(0)]]
var<out> out_color: vec3<f32>;

[[block]]
struct Sky {
    start_color: vec3<f32>;
    end_color: vec3<f32>;
};
[[group(0), binding(1)]]
var r_sky: Sky;

[[stage(vertex)]]
fn vs_main() {
	out_color = r_sky.start_color;
	out_pos = vec4<f32>(f32(1 - i32(in_vertex_index)) * 0.5, f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5, 0.0, 1.0);
}

[[location(0)]]
var<out> out_color: vec4<f32>;

[[location(0)]]
var<in> in_color_fs: vec3<f32>;

[[stage(fragment)]]
fn fs_main() {
    out_color = vec4<f32>(in_color_fs.x, in_color_fs.y, in_color_fs.z, 1.0);
}