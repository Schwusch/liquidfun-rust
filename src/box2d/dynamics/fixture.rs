use std::mem::transmute;
use libc::{size_t, c_void};
use super::super::collision::shapes::shape;
use super::super::common::settings::*;
use super::body::*;

/// This holds contact filtering data.
#[repr(C)]
#[derive(Debug)]
pub struct Filter {
    /// The collision category bits. Normally you would just set one bit.
    pub category_bits: UInt16,

    /// The collision mask bits. This states the categories that this
    /// shape would accept for collision.
    pub mask_bits: UInt16,

    /// Collision groups allow a certain group of objects to never collide (negative)
    /// or always collide (positive). Zero means no collision group. Non-zero group
    /// filtering always wins against the mask bits.
    pub group_index: Int16,
}

impl Default for Filter {
	fn default() -> Filter {
		Filter {
            category_bits: 0x0001,
            mask_bits: 0xFFFF,
            group_index: 0
        }		
	}
}

/// A fixture definition is used to create a fixture. This class defines an
/// abstract fixture definition. You can reuse fixture definitions safely.
#[repr(C)]
pub struct FixtureDef {
    /// The shape, this must be set. The shape will be cloned, so you
    /// can create the shape on the stack.
    pub shape: *mut shape::B2Shape,

    /// Use this to store application specific fixture data.
    pub user_data: size_t,

    /// The friction coefficient, usually in the range [0,1].    
    pub friction: Float32,

    /// The restitution (elasticity) usually in the range [0,1].
    pub restitution: Float32,

    /// The density, usually in kg/m^2.
    pub density: Float32,

    /// A sensor shape collects contact information but never generates a collision
    /// response.
    pub is_sensor: bool,

    /// Contact filtering data.
    pub filter: Filter
}

impl FixtureDef {
    pub fn new(shape: &shape::Shape) -> FixtureDef {
        FixtureDef {
            shape: shape.handle(),
            user_data: 0,
            friction: 0.0,
            restitution: 0.0,
            density: 0.0,
            is_sensor: false,
            filter: Filter::default()
        }
    }
}

pub enum B2Fixture {}

extern {
    fn b2Fixture_GetNext(this: *mut B2Fixture) -> *mut B2Fixture;
    fn b2Fixture_GetShape(this: *mut B2Fixture) -> *mut shape::B2Shape;
    fn b2Fixture_GetType(this: *mut B2Fixture) -> shape::Type;
    fn b2Fixture_GetUserData(this: *mut B2Fixture) -> *mut c_void;
    fn b2Fixture_SetUserData(this: *mut B2Fixture, data: *mut c_void);
    fn b2Fixture_GetBody(this: *mut B2Fixture) -> *mut B2Body;
}

/// A fixture is used to attach a shape to a body for collision detection. A fixture
/// inherits its transform from its parent. Fixtures hold additional non-geometric data
/// such as friction, collision filters, etc.
/// Fixtures are created via b2Body::CreateFixture.
/// @warning you cannot reuse fixtures.
#[allow(raw_pointer_derive)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Fixture {
    pub masked_ptr: usize
}

impl Fixture {

    /// Get the type of the child shape. You can use this to down cast to the concrete shape.
    /// @return the shape type.
    pub fn get_type(&self) -> shape::Type {
        unsafe {
            b2Fixture_GetType(self.get_ptr())
        }
    }

    /// Get the child shape. You can modify the child shape, however you should not change the
    /// number of vertices because this will crash some collision caching mechanisms.
    /// Manipulating the shape may lead to non-physical behavior.
    pub fn get_shape(&self) -> *mut shape::B2Shape {
        unsafe {
            return b2Fixture_GetShape(self.get_ptr());
        }
    }    

    /// Get the next fixture in the parent body's fixture list.
    /// @return the next fixture.
    pub fn get_next(&self) -> Option<Fixture> {
        let ptr: *mut B2Fixture;
        
        unsafe {
            ptr = b2Fixture_GetNext(self.get_ptr());
        }

        if ptr.is_null() {
            None
        } else {
            Some(Fixture { masked_ptr: ptr as usize })
        }        
    }


    pub fn get_user_data(&mut self) -> usize {
        unsafe {
            transmute(b2Fixture_GetUserData(self.get_ptr()))
        }
    }

    pub fn set_user_data(&mut self, data: usize) {
        unsafe {
            b2Fixture_SetUserData(self.get_ptr(), transmute(data));
        }
    }

    pub fn get_body(&mut self) -> Body {
        unsafe {
            Body { masked_ptr: b2Fixture_GetBody(self.get_ptr()) as usize }
        }
    }

    pub fn get_ptr(&self) -> *mut B2Fixture {
        self.masked_ptr as *mut B2Fixture

    }
}
