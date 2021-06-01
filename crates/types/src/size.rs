/// Defines a simple `Size` data structure.
pub struct Size<T> {
	pub width: T,
	pub height: T
}

impl<T> Size<T> {
	/// Creates and return a new `Size` which contains
	/// the provided width and height.
	pub fn new(width: T, height: T) -> Self {
		Self {
			width,
			height
		}
	}
}