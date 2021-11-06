use std::ffi::c_void;

use super::{open, close};

#[cxx::bridge]
mod plugin {
	extern "C++" {
		include!("plugin.hpp");
		unsafe fn CreateInterface() -> usize;
	}

	extern "Rust" {
		fn open();
		fn close();
	}
}

#[no_mangle]
extern "C" fn CreateInterface() -> *mut c_void {
	unsafe { plugin::CreateInterface() as *mut c_void }
}