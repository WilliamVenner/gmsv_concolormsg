#![feature(c_variadic)]

#[cfg(not(all(target_os = "windows", target_pointer_width = "64")))]
compile_error!("Unsupported platform");

#[macro_use]
extern crate gmod;

mod plugin;
mod colormsg;
mod detour;

fn open() {
	#[cfg(target_os = "windows")]
	if let Err(err) = ansi_term::enable_ansi_support() {
		eprintln!("gmsv_concolormsg | FAILED TO ENABLE ANSI COLOR SUPPORT (Error {})", err);
		return;
	}

	unsafe { detour::detour() };
}

fn close() {
	unsafe { detour::revert() };
}