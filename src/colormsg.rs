use std::os::raw::c_char;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Color {
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

const MAXPRINTMSG: usize = 5020;

extern "C" {
	pub fn vsnprintf(
		__s: *mut ::std::os::raw::c_char,
		__maxlen: usize,
		__format: *const ::std::os::raw::c_char,
		__arg: std::ffi::VaList,
	) -> ::std::os::raw::c_int;
}
macro_rules! va_fmt_to_string {
	($str:ident, $fmt:ident, $args:ident) => {
		let mut buf = [0i8; MAXPRINTMSG];
		let len = vsnprintf(buf.as_mut_ptr(), MAXPRINTMSG - 1, $fmt, $args.as_va_list());
		if len == -1 { return; }
		let len = (len as usize).min(MAXPRINTMSG);
		let buf: [u8; MAXPRINTMSG] = std::mem::transmute(buf);
		let $str = String::from_utf8_lossy(&buf[0..len]);
	};
}

pub unsafe extern "C" fn con_color_msg(color: *const Color, fmt: *const c_char, mut args: ...) {
	use std::io::Write;

	va_fmt_to_string!(str, fmt, args);

	let color = &*color;
	let ansi_color = ansi_term::Color::RGB(color.r(), color.g(), color.b());

	let mut stdout = std::io::stdout();
	stdout.write_all(ansi_color.paint(str.as_ref()).to_string().as_bytes()).ok();
	stdout.flush().ok(); // If we don't flush, the terminal sometimes doesn't reset the style.
}