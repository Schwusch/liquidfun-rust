// use std::mem::transmute;
// use libc::{size_t, c_void};
// use super::super::super::collision::shapes::shape;
// use super::super::super::common::settings::*;
use super::super::fixture::*;

pub enum B2Contact {}

extern {
    fn b2Contact_GetFixtureA(this: *mut B2Contact) -> *mut B2Fixture;
    fn b2Contact_GetFixtureB(this: *mut B2Contact) -> *mut B2Fixture;
}

#[allow(raw_pointer_derive)]
#[derive(Clone)]
pub struct Contact {
    pub ptr: *mut B2Contact
}

impl Contact {
    pub fn get_fixture_a(&mut self) -> Fixture {
        unsafe {
            Fixture { masked_ptr: b2Contact_GetFixtureA(self.ptr) as usize}
        }
    }

    pub fn get_fixture_b(&mut self) -> Fixture {
        unsafe {
            Fixture { masked_ptr: b2Contact_GetFixtureB(self.ptr) as usize }
        }
    }
}