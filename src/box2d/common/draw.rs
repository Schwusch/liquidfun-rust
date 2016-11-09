#![allow(non_snake_case)]

use std::slice::from_raw_parts;

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

/// Implement and register this trait with a World to provide debug drawing of physics
/// using World.set_debug_draw()
pub trait Draw {
	fn set_flags(&mut self, flags: UInt32);
	fn get_flags(&self) -> UInt32;
	fn append_flags(&mut self, flags: UInt32);
	fn clear_flags(&mut self, flags: UInt32);
	fn draw_polygon(&mut self, vertices: &[Vec2], color: &Color);
	fn draw_solid_polygon(&mut self, vertices: &[Vec2], color: &Color);
	fn draw_circle(&mut self, center: &Vec2, radius: f32, color: &Color);
	fn draw_solid_circle(&mut self, center: &Vec2, radius: f32, axis: &Vec2, color: &Color);
	fn draw_particles(&mut self, centers: &[Vec2], colors: &[ParticleColor], radius: f32);
	fn draw_segment(&mut self, p1: &Vec2, p2: &Vec2, color: &Color);
	fn draw_transform(&mut self, xf: &Transform);
}

pub type BoxDebugDraw = Box<Draw>;

#[no_mangle]
pub extern fn BoxDebugDraw_SetFlags(this: *mut BoxDebugDraw, flags: UInt32) {
	unsafe {
		(*this).set_flags(flags);
	}
}

#[no_mangle]
pub extern fn BoxDebugDraw_GetFlags(this: *mut BoxDebugDraw) -> UInt32 {
	unsafe {
		(*this).get_flags()
	}
}

#[no_mangle]
pub extern fn BoxDebugDraw_AppendFlags(this: *mut BoxDebugDraw, flags: UInt32) {
	unsafe {
		(*this).append_flags(flags);
	}
}

#[no_mangle]
pub extern fn BoxDebugDraw_ClearFlags(this: *mut BoxDebugDraw, flags: UInt32) {
	unsafe {
		(*this).clear_flags(flags);
	}
}

#[no_mangle]
pub extern fn BoxDebugDraw_DrawPolygon(this: *mut BoxDebugDraw, vertices: *const Vec2, vertexCount: Int32, color: &Color) {
	unsafe {
		(*this).draw_polygon(from_raw_parts(vertices, vertexCount as usize), color);
	}
}

#[no_mangle]
pub extern fn BoxDebugDraw_DrawSolidPolygon(this: *mut BoxDebugDraw, vertices: *const Vec2, vertexCount: Int32, color: &Color) {
	unsafe {
		(*this).draw_solid_polygon(from_raw_parts(vertices, vertexCount as usize), color);
	}
}

#[no_mangle]
pub extern fn BoxDebugDraw_DrawCircle(this: *mut BoxDebugDraw, center: &Vec2, radius: f32, color: &Color) {
	unsafe {
		(*this).draw_circle(center, radius, color);
	}
}

#[no_mangle]
pub extern fn BoxDebugDraw_DrawSolidCircle(this: *mut BoxDebugDraw, center: &Vec2, radius: f32, axis: &Vec2, color: &Color) {
	unsafe {
		(*this).draw_solid_circle(center, radius, axis, color);
	}
}

#[no_mangle]
pub extern fn BoxDebugDraw_DrawParticles(this: *mut BoxDebugDraw, centers: *const Vec2, radius: f32, colors: *const ParticleColor, count: Int32) {
	unsafe {
		(*this).draw_particles(from_raw_parts(centers, count as usize), from_raw_parts(colors, count as usize), radius);
	}
}

#[no_mangle]
pub extern fn BoxDebugDraw_DrawSegment(this: *mut BoxDebugDraw, p1: &Vec2, p2: &Vec2, color: &Color) {
	unsafe {
		(*this).draw_segment(p1, p2, color);
	}
}

#[no_mangle]
pub extern fn BoxDebugDraw_DrawTransform(this: *mut BoxDebugDraw, xf: &Transform) {
	unsafe {
		(*this).draw_transform(xf);
	}
}
