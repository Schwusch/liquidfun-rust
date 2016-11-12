use super::super::common::settings::*;

/// Small color object for each particle
/// Four elements: r (red), g (green), b (blue), and a (opacity).
/// Each element can be specified 0 to 255.
#[repr(C)]
#[derive(new, Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ParticleColor { pub r: u8, pub g: u8, pub b: u8, pub a: u8 }

pub enum B2ParticleColor {}

extern {
	fn b2ParticleColor_New(r: UInt8, g: UInt8, b: UInt8, a: UInt8) -> *mut B2ParticleColor;
	fn b2ParticleColor_Delete(p: *mut B2ParticleColor);
    fn b2ParticleColor_Set(p: *mut B2ParticleColor, r: UInt8, g: UInt8, b: UInt8, a: UInt8);
    fn b2ParticleColor_IsZero(p: *mut B2ParticleColor) -> bool;
}


/// Small color object for each particle
#[allow(raw_pointer_derive)]
#[derive(Clone)]
pub struct ParticleColorHandle {
	pub ptr: *mut B2ParticleColor
}

impl ParticleColorHandle {

    /// Construct a new ParticleColorHandle from a ParticleColor
    pub fn new(c: &ParticleColor) -> ParticleColorHandle {
        unsafe {
            ParticleColorHandle { ptr: b2ParticleColor_New(c.r, c.g, c.b, c.a) }
        }
    }

    /// Create a ParticleColorHandle with zero values.
	pub fn zero() -> ParticleColorHandle {
		ParticleColorHandle::new(&ParticleColor::default())
	}

    /// True when all four color elements equal 0. When true, a particle color
    /// buffer isn't allocated by CreateParticle().
    pub fn is_zero(&self) -> bool {
        unsafe {
            b2ParticleColor_IsZero(self.ptr)
        }
    }

    /// Get ParticleColorHandle's raw pointer.
	pub fn ptr(&self) -> *mut B2ParticleColor {
		self.ptr
	}

    /// Sets color for current object using the four elements described above.
    pub fn set(&self, r: UInt8, g: UInt8, b: UInt8, a: UInt8) {
        unsafe {
            b2ParticleColor_Set(self.ptr, r, g, b, a);
        }
    }

}

impl Drop for ParticleColorHandle {
    fn drop(&mut self) {
        unsafe {
            b2ParticleColor_Delete(self.ptr);
        }
    }
}