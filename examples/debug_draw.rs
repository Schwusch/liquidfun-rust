#![feature(box_syntax)]
use std::any::Any;

extern crate libc;
extern crate liquidfun;

#[macro_use]
extern crate bitflags;

use liquidfun::box2d::collision::shapes::polygon_shape::*;
use liquidfun::box2d::common::math::*;
use liquidfun::box2d::dynamics::body::*;
use liquidfun::box2d::dynamics::fixture::*;
use liquidfun::box2d::dynamics::world::*;
use liquidfun::box2d::common::draw::*;
use liquidfun::box2d::particle::particle_color::ParticleColor;

fn main() {

	// Define the gravity vector.
	let gravity = Vec2::new(0.0, -10.0);

	// Construct a world object, which will hold and simulate the rigid bodies.
	let mut world = World::new(&gravity);

	// Define the ground body.
	let mut ground_body_def = BodyDef::default();
	ground_body_def.position.set(0.0, -10.0);

	// Call the body factory which allocates memory for the ground body
	// from a pool and creates the ground box shape (also from a pool).
	// The body is also added to the world.
	let mut ground_body = world.create_body(&ground_body_def);

	// Define the ground box shape.
	let mut ground_box = PolygonShape::new();

	// The extents are the half-widths of the box.
	ground_box.set_as_box(50.0, 10.0);

	// Add the ground fixture to the ground body.
	ground_body.create_fixture_from_shape(&ground_box, 0.0);

	// Define the dynamic body. We set its position and call the body factory.
	let mut body_def = BodyDef::default();
	body_def.body_type = BodyType::DynamicBody;
	body_def.position.set(0.0, 4.0);
	let mut body = world.create_body(&body_def);

	// Define another box shape for our dynamic body.
	let mut dynamic_box = PolygonShape::new();
	dynamic_box.set_as_box(1.0, 1.0);

	// Define the dynamic body fixture.
	let mut fixture_def = FixtureDef::new(&dynamic_box);

	// Set the box density to be non-zero, so it will be dynamic.
	fixture_def.density = 1.0;

	// Override the default friction.
	fixture_def.friction = 0.3;

	// Add the shape to the body.
	body.create_fixture(&fixture_def);

	// Register a routine for debug drawing. The debug draw functions are called
	// inside with World::draw_debug_data() method. The debug draw object is owned
	// by you and must remain in scope.
	let mut debug_draw = DebugDraw::new(MyDebugDraw::new());
	debug_draw.get().scale = 10.;
	world.set_debug_draw(&mut debug_draw);
	debug_draw.set_flags(DrawFlags::SHAPE);

	world.draw_debug_data();

	/* this would crash:
		{
			let mut debug_draw = DebugDraw::new(MyDebugDraw::new());
			debug_draw.get().scale = 10.;
			world.set_debug_draw(&mut debug_draw);
			debug_draw.set_flags(!0);
		} // debug_draw goes out of scope here but world still has pointer to it

		world.draw_debug_data();

		// if debug_draw goes out of scope before world, call world.clear_debug_draw()
	*/
}

struct MyDebugDraw {
	scale: f32,
}

impl MyDebugDraw {
	pub fn new() -> Self {
		Self {
			scale: 1.,
		}
	}
}

impl Drop for MyDebugDraw {
	fn drop(&mut self) {
		println!("drop MyDebugDraw");
	}
}

impl Draw for MyDebugDraw {
	fn as_any(&mut self) -> &mut Any { self }
	fn draw_polygon(&mut self, vertices: &[Vec2], color: &Color) {
		println!("self.scale {}", self.scale);
		println!("draw_polygon {:?} {:?}", vertices, color);
	}
	fn draw_solid_polygon(&mut self, vertices: &[Vec2], color: &Color) {
		println!("draw_solid_polygon {:?} {:?}", vertices, color);
	}
	fn draw_circle(&mut self, center: &Vec2, radius: f32, color: &Color) {
		println!("draw_circle {:?} {:?} {:?}", center, radius, color);
	}
	fn draw_solid_circle(&mut self, center: &Vec2, radius: f32, axis: &Vec2, color: &Color) {
		println!("draw_solid_circle {:?} {:?} {:?} {:?}", center, radius, axis, color);
	}
	fn draw_particles(&mut self, centers: &[Vec2], colors: &[ParticleColor], radius: f32) {
		println!("draw_particles {:?} {:?} {:?}", centers, colors, radius);
	}
	fn draw_segment(&mut self, p1: &Vec2, p2: &Vec2, color: &Color) {
		println!("draw_segment {:?} {:?} {:?}", p1, p2, color);
	}
	fn draw_transform(&mut self, xf: &Transform) {
		println!("draw_transform {:?}", xf);
	}
}
