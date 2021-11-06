#![feature(c_variadic)]

#[cfg(not(all(target_os = "windows", target_pointer_width = "64")))]
compile_error!("Unsupported platform");

#[macro_use]
extern crate gmod;

mod plugin;

use std::{cell::UnsafeCell, os::raw::c_char};

#[derive(Clone, Copy, Debug)]
#[repr(C)]
struct Color {
	_color: [std::os::raw::c_uchar; 4]
}
impl Color {
	#[inline(always)]
	pub const fn r(&self) -> u8 {
		self._color[0] as u8
	}

	#[inline(always)]
	pub const fn g(&self) -> u8 {
		self._color[1] as u8
	}

	#[inline(always)]
	pub const fn b(&self) -> u8 {
		self._color[2] as u8
	}
}

const MAXPRINTMSG: usize = 4096;

extern "C" {
	pub fn vsnprintf(
		__s: *mut ::std::os::raw::c_char,
		__maxlen: usize,
		__format: *const ::std::os::raw::c_char,
		__arg: *mut std::ffi::VaList,
	) -> ::std::os::raw::c_int;
}
macro_rules! va_fmt_to_string {
	($str:ident, $fmt:ident, $args:ident) => {
		let mut buf = [0i8; MAXPRINTMSG];
		let len = vsnprintf(buf.as_mut_ptr(), MAXPRINTMSG, $fmt, &mut $args.as_va_list() as *mut _);
		let buf: [u8; MAXPRINTMSG] = std::mem::transmute(buf);
		let $str = String::from_utf8_lossy(&buf[0..len as usize]);
	};
}

unsafe extern "C" fn con_color_msg(color: *const Color, fmt: *const c_char, mut args: std::ffi::VaListImpl<'static>) {
	use std::io::Write;

	va_fmt_to_string!(str, fmt, args);

	let color = &*color;
	let ansi_color = ansi_term::Color::RGB(color.r(), color.g(), color.b());
	print!("{}", ansi_color.paint(str.as_ref()));

	std::io::stdout().flush().ok(); // If we don't flush, the terminal sometimes doesn't reset the style.
}

static mut DETOURS: Detours = Detours::default();
struct Detours {
	con_color_msg: UnsafeCell<Option<gmod::detour::RawDetour>>
}
impl Detours {
	pub const fn default() -> Detours {
		Detours {
			con_color_msg: UnsafeCell::new(None)
		}
	}

	pub fn take(&mut self) -> Detours {
		std::mem::replace(self, Detours::default())
	}
}
unsafe impl Sync for Detours {}

macro_rules! add_detour {
	($name:ident, $detour:expr) => {
		(&mut *DETOURS.$name.get()).replace($detour);
	};
}

fn open() {
	#[cfg(target_os = "windows")]
	if let Err(err) = ansi_term::enable_ansi_support() {
		eprintln!("gmsv_concolormsg | FAILED TO ENABLE ANSI COLOR SUPPORT (Error {})", err);
	}

	println!("gmsv_concolormsg | Detouring...");

	unsafe {
		let (tier0, _tier0_path) = open_library_srv!("tier0").expect("Failed to open tier0");

		{
			let detour = gmod::detour::RawDetour::new(
				*tier0.get(b"?ConColorMsg@@YAXAEBVColor@@PEBDZZ\0").expect("Failed to find ConColorMsg"),
				con_color_msg as *const ()
			).expect("Failed to detour ConColorMsg");

			detour.enable().expect("Failed to enable ConColorMsg detour");
			add_detour!(con_color_msg, detour);
		}
	}

	println!("gmsv_concolormsg | Success!");
}

fn close() {
	println!("gmsv_concolormsg | Resetting...");

	unsafe { &mut DETOURS }.take();
}