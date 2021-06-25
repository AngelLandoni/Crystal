#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Color<T> {
	pub r: T,
	pub g: T,
	pub b: T
}