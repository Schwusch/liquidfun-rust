#![allow(non_snake_case)]

use std::slice::from_raw_parts;
use std::any::Any;

use super::math::*;
use super::settings::*;
use super::super::particle::particle_color::ParticleColor;

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

/// Implement and register this trait with a World to provide debug drawing of physics
/// using World.set_debug_draw()
pub trait Draw {

	/// Put this in your impl: fn as_any(&mut self) -> &mut Any { self }
	fn as_any(&mut self) -> &mut Any;

	/// Draw a closed polygon provided in CCW order.
	fn draw_polygon(&mut self, vertices: &[Vec2], color: &Color);
	
	/// Draw a solid closed polygon provided in CCW order.
	fn draw_solid_polygon(&mut self, vertices: &[Vec2], color: &Color);

	/// Draw a circle.
	fn draw_circle(&mut self, center: &Vec2, radius: f32, color: &Color);

	/// Draw a solid circle.
	fn draw_solid_circle(&mut self, center: &Vec2, radius: f32, axis: &Vec2, color: &Color);

	/// Draw a particle array
	fn draw_particles(&mut self, centers: &[Vec2], colors: &[ParticleColor], radius: f32);

	/// Draw a line segment.
	fn draw_segment(&mut self, p1: &Vec2, p2: &Vec2, color: &Color);

	/// Draw a transform. Choose your own length scale.
	fn draw_transform(&mut self, xf: &Transform);
}

pub type DrawTrait = Box<Draw>;

#[no_mangle]
pub extern fn DrawTrait_DrawPolygon(this: *mut DrawTrait, vertices: *const Vec2, vertexCount: Int32, color: &Color) {
	unsafe {
		(**this).draw_polygon(from_raw_parts(vertices, vertexCount as usize), color);
	}
}

#[no_mangle]
pub extern fn DrawTrait_DrawSolidPolygon(this: *mut DrawTrait, vertices: *const Vec2, vertexCount: Int32, color: &Color) {
	unsafe {
		(**this).draw_solid_polygon(from_raw_parts(vertices, vertexCount as usize), color);
	}
}

#[no_mangle]
pub extern fn DrawTrait_DrawCircle(this: *mut DrawTrait, center: &Vec2, radius: f32, color: &Color) {
	unsafe {
		(**this).draw_circle(center, radius, color);
	}
}

#[no_mangle]
pub extern fn DrawTrait_DrawSolidCircle(this: *mut DrawTrait, center: &Vec2, radius: f32, axis: &Vec2, color: &Color) {
	unsafe {
		(**this).draw_solid_circle(center, radius, axis, color);
	}
}

#[no_mangle]
pub extern fn DrawTrait_DrawParticles(this: *mut DrawTrait, centers: *const Vec2, radius: f32, colors: *const ParticleColor, count: Int32) {
	unsafe {
		(**this).draw_particles(from_raw_parts(centers, count as usize), from_raw_parts(colors, count as usize), radius);
	}
}

#[no_mangle]
pub extern fn DrawTrait_DrawSegment(this: *mut DrawTrait, p1: &Vec2, p2: &Vec2, color: &Color) {
	unsafe {
		(**this).draw_segment(p1, p2, color);
	}
}

#[no_mangle]
pub extern fn DrawTrait_DrawTransform(this: *mut DrawTrait, xf: &Transform) {
	unsafe {
		(**this).draw_transform(xf);
	}
}
