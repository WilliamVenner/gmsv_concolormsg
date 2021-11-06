use std::cell::UnsafeCell;

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

pub(super) unsafe fn detour() {
	println!("gmsv_concolormsg | Detouring...");

	{
		let (tier0, _tier0_path) = open_library_srv!("tier0").expect("Failed to open tier0");

		let detour = gmod::detour::RawDetour::new(
			*tier0.get(b"?ConColorMsg@@YAXAEBVColor@@PEBDZZ\0").expect("Failed to find ConColorMsg"),
			crate::colormsg::con_color_msg as *const ()
		).expect("Failed to detour ConColorMsg");

		detour.enable().expect("Failed to enable ConColorMsg detour");
		add_detour!(con_color_msg, detour);
	}

	println!("gmsv_concolormsg | Success!");
}

pub(super) unsafe fn revert() {
	(&mut DETOURS).take();
}