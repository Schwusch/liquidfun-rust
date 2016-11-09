use super::super::common::math::*;
use super::super::common::settings::*;

/// Flags for specifying what to draw, using set_flags()
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DrawFlags {
	/// draw shapes
	ShapeBit        = 0x0001,
	
	/// draw joint connections
	JointBit        = 0x0002,

	/// draw axis aligned bounding boxes
	AabbBit         = 0x0004,

	/// draw broad-phase pairs
	PairBit	        = 0x0008,

	/// draw center of mass frame
	CenterOfMassBit	= 0x0010,

	/// draw particles
	ParticleBit     = 0x0020,
}

/// Color for debug drawing. Each value has the range [0,1].
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Color { pub r: f32, pub g: f32, pub b: f32 }

/// Small color object for each particle
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ParticleColor { pub r: u8, pub g: u8, pub b: u8, pub a: u8 }

pub enum B2Draw {}

extern {
	fn b2Draw_SetFlags(this: *mut B2Draw, flags: UInt32);
	fn b2Draw_GetFlags(this: *const B2Draw) -> UInt32;
	fn b2Draw_AppendFlags(this: *mut B2Draw, flags: UInt32);
	fn b2Draw_ClearFlags(this: *mut B2Draw, flags: UInt32);
	fn b2Draw_DrawPolygon(this: *mut B2Draw, vertices: *const Vec2, vertexCount: Int32, color: &Color);
	fn b2Draw_DrawSolidPolygon(this: *mut B2Draw, vertices: *const Vec2, vertexCount: Int32, color: &Color);
	fn b2Draw_DrawCircle(this: *mut B2Draw, center: &Vec2, radius: f32, color: &Color);
	fn b2Draw_DrawSolidCircle(this: *mut B2Draw, center: &Vec2, radius: f32, axis: &Vec2, color: &Color);
	fn b2Draw_DrawParticles(this: *mut B2Draw, centers: *const Vec2, radius: f32, colors: *const ParticleColor, count: Int32);
	fn b2Draw_DrawSegment(this: *mut B2Draw, p1: &Vec2, p2: &Vec2, color: &Color);
	fn b2Draw_DrawTransform(this: *mut B2Draw, xf: &Transform);
}

/// Implement and register this class with a b2World to provide debug drawing of physics
/// entities in your game.
#[allow(raw_pointer_derive)]
#[derive(Clone, Debug)]
pub struct Draw {
	pub ptr: *mut B2Draw
}

impl Draw {

	/// Set the drawing flags.
	pub fn set_flags(&mut self, flags: UInt32) {
		unsafe {
			b2Draw_SetFlags(self.ptr, flags);
		}
	}

	/// Get the drawing flags.
	pub fn get_flags(&self) -> UInt32 {
		unsafe {
			b2Draw_GetFlags(self.ptr)
		}
	}

	/// Append flags to the current flags.
	pub fn append_flags(&mut self, flags: UInt32) {
		unsafe {
			b2Draw_AppendFlags(self.ptr, flags);
		}
	}

	/// Clear flags from the current flags.
	pub fn clear_flags(&mut self, flags: UInt32) {
		unsafe {
			b2Draw_ClearFlags(self.ptr, flags);
		}
	}

	/// Draw a closed polygon provided in CCW order.
	pub fn draw_polygon(&mut self, vertices: &[Vec2], color: &Color) {
		unsafe {
			b2Draw_DrawPolygon(self.ptr, vertices.as_ptr(), vertices.len() as Int32, color);
		}
	}

	/// Draw a solid closed polygon provided in CCW order.
	pub fn draw_solid_polygon(&mut self, vertices: &[Vec2], color: &Color) {
		unsafe {
			b2Draw_DrawSolidPolygon(self.ptr, vertices.as_ptr(), vertices.len() as Int32, color);
		}
	}

	/// Draw a circle.
	pub fn draw_circle(&mut self, center: &Vec2, radius: f32, color: &Color) {
		unsafe {
			b2Draw_DrawCircle(self.ptr, center, radius, color);
		}
	}

	/// Draw a solid circle.
	pub fn draw_solid_circle(&mut self, center: &Vec2, radius: f32, axis: &Vec2, color: &Color) {
		unsafe {
			b2Draw_DrawSolidCircle(self.ptr, center, radius, axis, color);
		}
	}

	/// Draw a particle array
	/// centers and colors must have the same length
	pub fn draw_particles(&mut self, centers: &[Vec2], radius: f32, colors: &[ParticleColor]) {
		// assert_eq!(centers.len(), colors.len());
		unsafe {
			b2Draw_DrawParticles(self.ptr, centers.as_ptr(), radius, colors.as_ptr(), colors.len() as Int32);
		}
	}

	/// Draw a line segment.
	pub fn draw_segment(&mut self, p1: &Vec2, p2: &Vec2, color: &Color) {
		unsafe {
			b2Draw_DrawSegment(self.ptr, p1, p2, color);
		}
	}

	/// Draw a transform. Choose your own length scale.
	pub fn draw_transform(&mut self, xf: &Transform) {
		unsafe {
			b2Draw_DrawTransform(self.ptr, xf);
		}
	}
}
