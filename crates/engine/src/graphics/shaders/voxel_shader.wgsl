[[location(0)]]
var<in> in_position: vec4<f32>;
[[location(1)]]
var<in> in_vertex_color: vec4<f32>;
[[location(2)]]
var<in> in_uv: vec4<f32>;

[[location(3)]]
var<in> in_color: vec3<f32>;

// Get the transformation matrix using 4 4D vectors.
[[location(4)]]
var<in> in_transform_0: vec4<f32>;
[[location(5)]]
var<in> in_transform_1: vec4<f32>;
[[location(6)]]
var<in> in_transform_2: vec4<f32>;
[[location(7)]]
var<in> in_transform_3: vec4<f32>;

[[builtin(position)]]
var<out> out_pos: vec4<f32>;
[[location(0)]]
var<out> out_color: vec3<f32>;

[[block]]
struct Locals {
    view: mat4x4<f32>;
    projection: mat4x4<f32>;
};
[[group(0), binding(0)]]
var r_locals: Locals;

[[stage(vertex)]]
fn vs_main() {
	// Try to find a better way to create this matrix.
	const entity_transform: mat4x4<f32> = mat4x4<f32>(
		in_transform_0.x, in_transform_0.y, in_transform_0.z, in_transform_0.w,
		in_transform_1.x, in_transform_1.y, in_transform_1.z, in_transform_1.w,
		in_transform_2.x, in_transform_2.y, in_transform_2.z, in_transform_2.w,
		in_transform_3.x, in_transform_3.y, in_transform_3.z, in_transform_3.w
	);

	out_color = vec3<f32>(in_color.x, in_color.y, in_color.z);//in_color;

	// Recreate the transformation matrix.
	out_pos = r_locals.projection * r_locals.view * entity_transform * in_position;
}

[[location(0)]]
var<in> in_color_fs: vec3<f32>;
[[location(0)]]
var<out> out_color: vec4<f32>;

[[stage(fragment)]]
fn fs_main() {
    out_color = vec4<f32>(in_color_fs.x, in_color_fs.y, in_color_fs.z, 1.0);
}
