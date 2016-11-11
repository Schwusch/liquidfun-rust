#![allow(non_snake_case)]

use std::any::Any;

use super::contacts::contact::*;

pub trait ContactListener {

	/// Put this in your impl: fn as_any(&mut self) -> &mut Any { self }
	fn as_any(&mut self) -> &mut Any;
	fn begin_contact(&mut self, contact: &mut Contact);
	fn end_contact(&mut self, contact: &mut Contact);
}

pub type ContactListenerTrait = Box<ContactListener>;

#[no_mangle]
pub extern fn ContantListenerTrait_BeginContact(this: *mut ContactListenerTrait, contact: *mut B2Contact) {
	unsafe {
		(*this).begin_contact(&mut Contact { ptr: contact });
	}
}

#[no_mangle]
pub extern fn ContantListenerTrait_EndContact(this: *mut ContactListenerTrait, contact: *mut B2Contact) {
	unsafe {
		(*this).end_contact(&mut Contact { ptr: contact });
	}
}
