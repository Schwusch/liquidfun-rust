use libc::size_t;
use std::ptr;
use std::marker::PhantomData;

use super::body::*;
use super::super::common::math::*;
use super::super::common::settings::*;
use super::super::particle::particle_system::*;
use super::joints;
use super::super::common::draw::*;
use super::world_callbacks::*;

pub enum B2World {}

extern {
	fn b2World_CreateBody(world: *mut B2World, bd: *const BodyDef) -> *mut B2Body;
    fn b2World_DestroyBody(world: *mut B2World, body: *mut B2Body);
	fn b2World_CreateParticleSystem(world: *mut B2World, def: *const ParticleSystemDef) -> *mut B2ParticleSystem;
	fn b2World_Delete(world: *mut B2World);
	fn b2World_GetBodyCount(world: *const B2World) -> Int32;
	fn b2World_GetJointCount(world: *const B2World) -> Int32;
	fn b2World_GetBodyList(world: *const B2World) -> *mut B2Body;
	fn b2World_GetGravity(world: *mut B2World) -> Vec2;
	fn b2World_GetParticleSystemList(world: *const B2World) -> *mut B2ParticleSystem;
	fn b2World_New(gravity: *const Vec2) -> *mut B2World;
	fn b2World_Step(this: *mut B2World, timeStep: Float32, velocityIterations: Int32, positionIterations: Int32);

	fn b2World_CreateRevoluteJoint(
		world: *mut B2World,
		
		joint_type: joints::JointType,
		user_data: size_t,
		body_a: *mut B2Body,
		body_b: *mut B2Body,
		collide_connected: bool,

		local_anchor_a: Vec2,
		local_anchor_b: Vec2,
		reference_angle: Float32,
		enable_limit: bool,
		lower_angle: Float32,
		upper_angle: Float32,
		enable_motor: bool,
		motor_speed: Float32,
		max_motor_torque: Float32
	) -> *mut joints::revolute_joint::B2RevoluteJoint;

	fn b2World_SetDebugDraw(this: *mut B2World, debug_draw: *mut CppDebugDraw);
	// fn b2World_GetDebugDraw(this: *mut B2World) -> *mut CppDebugDraw;
	fn b2World_DrawDebugData(this: *mut B2World);
	fn b2World_SetContactListener(this: *mut B2World, listener: *mut CppContactListener);
}

/// The world class manages all physics entities, dynamic simulation,
/// and asynchronous queries. The world also contains efficient memory
/// management facilities.
pub struct World {
	pub ptr: *mut B2World,
}

impl World {

	/// Construct a world object.
	/// @param gravity the world gravity vector.
	pub fn new(gravity: &Vec2) -> World {
		unsafe {
			World { ptr: b2World_New(gravity) }
		}
	}

	/// Create a rigid body given a definition. No reference to the definition
	/// is retained.
	/// @warning This function is locked during callbacks.
	pub fn create_body(&mut self, def: &BodyDef) -> Body {
		unsafe {
			Body { ptr: b2World_CreateBody(self.ptr, def) }
		}
	}

	pub fn destroy_body(&mut self, body: &Body) {
		unsafe {
			b2World_DestroyBody(self.ptr, body.ptr);
		}
	}

	/// Create a revolute joint to constrain bodies together. No reference to the definition
	/// is retained. This may cause the connected bodies to cease colliding.
	/// @warning This function is locked during callbacks.
	pub fn create_revolute_joint(&mut self, def: &(joints::JointDef, joints::revolute_joint::RevoluteJointDef)) -> joints::revolute_joint::RevoluteJoint {
		unsafe {
			joints::revolute_joint::RevoluteJoint {ptr: b2World_CreateRevoluteJoint(
				self.ptr,
				def.0.joint_type,
				def.0.user_data,
				match def.0.body_a {
					Some(ref b) =>b.ptr,
					None => ptr::null_mut()
				},
				match def.0.body_b {
					Some(ref b) =>b.ptr,
					None => ptr::null_mut()
				},
				def.0.collide_connected,
				def.1.local_anchor_a,
				def.1.local_anchor_b,
				def.1.reference_angle,
				def.1.enable_limit,
				def.1.lower_angle,
				def.1.upper_angle,
				def.1.enable_motor,
				def.1.motor_speed,
				def.1.max_motor_torque
			)}
		}
	}

	/// Create a particle system given a definition. No reference to the
	/// definition is retained.
	/// @warning This function is locked during callbacks.
	pub fn create_particle_system(&self, def: &ParticleSystemDef) -> ParticleSystem {
		unsafe {
			ParticleSystem { ptr: b2World_CreateParticleSystem(self.ptr, def) }
		}
	}

	/// Get the number of bodies.
	pub fn get_body_count(&self) -> i32 {
		unsafe {
			b2World_GetBodyCount(self.ptr)
		}
	}

	/// Get the number of joints.
	pub fn get_joint_count(&self) -> i32 {
		unsafe {
			b2World_GetJointCount(self.ptr)
		}
	}

	/// Get the world body list. With the returned body, use b2Body::GetNext to get
	/// the next body in the world list. A NULL body indicates the end of the list.
	/// @return the head of the world body list.
	pub fn get_body_list(&self) -> Option<Body> {
		let ptr;
		unsafe {
			ptr = b2World_GetBodyList(self.ptr);
		}

		if ptr.is_null() {
			None
		} else {
			Some(Body { ptr: ptr })
		}
	}

	/// Get the world particle-system list. With the returned body, use
	/// b2ParticleSystem::GetNext to get the next particle-system in the world
	/// list. A NULL particle-system indicates the end of the list.
	/// @return the head of the world particle-system list.
	pub fn get_particle_system_list(&self) -> Option<ParticleSystem> {
		let ptr;
		unsafe {
			ptr = b2World_GetParticleSystemList(self.ptr);
		}

		if ptr.is_null() {
			None
		} else {
			Some(ParticleSystem { ptr: ptr })
		}
	}

	/// Get the global gravity vector.
	pub fn get_gravity(&mut self) -> Vec2 {
		unsafe {
			b2World_GetGravity(self.ptr)
		}
	}

	/// Take a time step. This performs collision detection, integration,
	/// and constraint solution.
	/// @param timeStep the amount of time to simulate, this should not vary.
	/// @param velocityIterations for the velocity constraint solver.
	/// @param positionIterations for the position constraint solver.
	pub fn step(&mut self, time_step: f32, velocity_iterations: i32, position_iterations: i32) {
		unsafe {
			b2World_Step(self.ptr, time_step, velocity_iterations, position_iterations);
		}
	}

	/// Register a routine for debug drawing. The debug draw functions are called
	/// inside with World::draw_debug_data() method. The debug draw object is owned
	/// by you and must remain in scope.
	pub fn set_debug_draw<T: Draw + 'static>(&mut self, debug_draw: &mut DebugDraw<T>) {
		unsafe {
			b2World_SetDebugDraw(self.ptr, debug_draw.ptr);
		}
	}

	/// World stores a pointer to DebugDraw when you call World.set_debug_draw().
	/// You have to make sure your DebugDraw instance stays alive as long as the world.
	/// If the world outlives your DebugDraw instance, draw_debug_data() would dereference
	/// an invalid pointer. So call this function to set the internal DebugDraw pointer to null.
	pub fn clear_debug_draw(&mut self) {
		unsafe {
			b2World_SetDebugDraw(self.ptr, ptr::null_mut());
		}
	}

	/// Call this to draw shapes and other debug draw data. This is intentionally non-const.
	pub fn draw_debug_data(&mut self) {
		unsafe {
			b2World_DrawDebugData(self.ptr);
		}
	}

	pub fn set_contact_listener<T: ContactListener + 'static>(&mut self, listener: &mut ContactListenerHandle<T>) {
		unsafe {
			b2World_SetContactListener(self.ptr, listener.ptr);
		}
	}
}

impl Drop for World {
	fn drop(&mut self) {
		unsafe {
			b2World_Delete(self.ptr);
		}
	}
}

pub enum CppContactListener {}

#[allow(improper_ctypes)]
extern {
	fn CppContactListener_new(this: *mut ContactListenerTrait) -> *mut CppContactListener;
	fn CppContactListener_delete(this: *mut CppContactListener);
}

pub struct ContactListenerHandle<T: ContactListener + 'static> {
	trait_obj: *mut ContactListenerTrait,
	ptr: *mut CppContactListener,
	phantom: PhantomData<T>,
}

impl<T: ContactListener + 'static> ContactListenerHandle<T> {
	pub fn new(this: T) -> Self {
		let trait_obj = Box::into_raw(box (box this as Box<ContactListener>));
		Self {
			trait_obj: trait_obj,
			ptr: unsafe { CppContactListener_new(trait_obj) },
			phantom: PhantomData,
		}
	}

	/// Get direct access to your ContactListener instance.
	pub fn get(&mut self) -> &mut T {
		match unsafe { (*self.trait_obj).as_any().downcast_mut::<T>() } {
			Some(x) => x,
			None => panic!("invalid Box downcast")
		}
	}
}

impl<T: ContactListener + 'static> Drop for ContactListenerHandle<T> {
	fn drop(&mut self) {
		unsafe {
			CppContactListener_delete(self.ptr);
			drop(Box::from_raw(self.trait_obj));
		}
	}
}

pub enum CppDebugDraw {}

#[allow(improper_ctypes)]
extern {
	fn CppDebugDraw_new(this: *mut DrawTrait) -> *mut CppDebugDraw;
	fn CppDebugDraw_delete(this: *mut CppDebugDraw);
	fn CppDebugDraw_SetFlags(this: *mut CppDebugDraw, flags: UInt32);
	fn CppDebugDraw_GetFlags(this: *mut CppDebugDraw) -> UInt32;
	fn CppDebugDraw_AppendFlags(this: *mut CppDebugDraw, flags: UInt32);
	fn CppDebugDraw_ClearFlags(this: *mut CppDebugDraw, flags: UInt32);
}

/// Call DebugDraw::new() to create a new DebugDraw instance from an instance of your type
/// that implements the Draw trait and pass this to World::set_debug_draw()
pub struct DebugDraw<T: Draw + 'static> {
	trait_obj: *mut DrawTrait,
	ptr: *mut CppDebugDraw,
	phantom: PhantomData<T>,
}

impl<T: Draw + 'static> DebugDraw<T> {

	/// Creates a new DebugDraw instance.
	pub fn new(this: T) -> Self {
		let trait_obj = Box::into_raw(box (box this as Box<Draw>));
		Self {
			trait_obj: trait_obj,
			ptr: unsafe { CppDebugDraw_new(trait_obj) },
			phantom: PhantomData,
		}
	}

	/// Get direct access to your Draw instance.
	pub fn get(&mut self) -> &mut T {
		match unsafe { (*self.trait_obj).as_any().downcast_mut::<T>() } {
			Some(x) => x,
			None => panic!("invalid Box downcast")
		}
	}

	/// Set the drawing flags.
	pub fn set_flags(&mut self, flags: u32) {
		unsafe {
			CppDebugDraw_SetFlags(self.ptr, flags);
		}
	}

	/// Get the drawing flags.
	pub fn get_flags(&mut self) -> u32 {
		unsafe {
			CppDebugDraw_GetFlags(self.ptr)
		}
	}

	/// Append flags to the current flags.
	pub fn append_flags(&mut self, flags: u32) {
		unsafe {
			CppDebugDraw_AppendFlags(self.ptr, flags);
		}
	}

	/// Clear flags from the current flags.
	pub fn clear_flags(&mut self, flags: u32) {
		unsafe {
			CppDebugDraw_ClearFlags(self.ptr, flags);
		}
	}
}

impl<T: Draw + 'static> Drop for DebugDraw<T> {
	fn drop(&mut self) {
		unsafe {
			CppDebugDraw_delete(self.ptr);
			drop(Box::from_raw(self.trait_obj));
		}
	}
}
