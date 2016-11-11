use super::shape::*;
use super::super::super::common::settings::*;
use super::super::super::common::math::*;

enum B2CircleShape {}

extern {
    fn b2CircleShape_New() -> *mut B2CircleShape;
    fn b2CircleShape_Delete(this: *mut B2CircleShape);
    fn b2CircleShape_Upcast(ptr: *mut B2CircleShape) -> *mut B2Shape;
    fn b2CircleShape_SetPosition(this: *mut B2CircleShape, pos: &Vec2);
    fn b2CircleShape_SetRadius(this: *mut B2CircleShape, radius: Float32);
}

/// A circle
pub struct CircleShape {
    ptr: *mut B2CircleShape,
    owned: bool,    
}

impl Shape for CircleShape {
    // Is the up-cast even necessary? Can't we just use self.ptr directly?
    fn handle(&self) -> *mut B2Shape {
        unsafe {
            b2CircleShape_Upcast(self.ptr)
        }
    }
}

impl CircleShape {

    /// Create a new CircleShape.
    pub fn new() -> CircleShape {
        unsafe {
            CircleShape { ptr: b2CircleShape_New(), owned: true }
        }
    }

    /// Cast a CircleShape from a B2Shape.
    pub fn from_shape(ptr: *mut B2Shape) -> CircleShape {
        CircleShape { ptr: ptr as *mut B2CircleShape, owned: false}
    }

    pub fn set_position(&mut self, pos: &Vec2) {
        unsafe {
            b2CircleShape_SetPosition(self.ptr, pos);
        }
    }

    pub fn set_radius(&mut self, radius: f32) {
        unsafe {
            b2CircleShape_SetRadius(self.ptr, radius);
        }
    }
}

impl Drop for CircleShape {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                b2CircleShape_Delete(self.ptr);
            }
        }
    }
}