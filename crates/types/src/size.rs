/// Defines a simple `Size` data structure.
pub struct Size<T: Clone + Copy> {
	pub width: T,
	pub height: T
}

impl<T> Size<T> where T: Clone + Copy {
	/// Creates and return a new `Size` which contains
	/// the provided width and height.
	pub fn new(width: T, height: T) -> Self {
		Self {
			width,
			height
		}
	}
}

impl<T> Copy for Size<T> where T: Clone + Copy {}

impl<T> Clone for Size<T> where T: Clone + Copy {
	fn clone(&self) -> Self {
		*self
	}
}