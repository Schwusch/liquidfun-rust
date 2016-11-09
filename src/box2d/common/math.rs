use super::settings::*;

/// A 2D column vector.
#[repr(C)]
#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct Vec2 {
    pub x: Float32,
    pub y: Float32,
}

impl Vec2 {

	/// Construct using coordinates.
	pub fn new(x: f32, y: f32) -> Vec2 {
		Vec2 {x: x, y: y}
	}

	/// Set this vector to some specified coordinates.
	pub fn set(&mut self, x: f32, y: f32) {
		self.x = x;
		self.y = y;
	}

	/// Construct a vector with all zero coordinates.
	pub fn zero() -> Vec2 {
		Vec2::default()
	}
}

/// A transform contains translation and rotation. It is used to represent
/// the position and orientation of rigid frames.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Transform { pub p: Vec2, pub q: Rot }

/// Rotation
/// Sine and cosine
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Rot { pub s: f32, pub c: f32 }
