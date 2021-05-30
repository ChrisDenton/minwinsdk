use std::{io, iter::once, path::PathBuf, ptr, slice};

mod imports;
pub use imports::LIBS;

/// A generic Windows GUID.
#[repr(C)]
pub struct Guid {
	pub data1: u32,
	pub data2: u16,
	pub data3: u16,
	pub data4: [u8; 8],
}
#[allow(non_snake_case)]
pub const fn Guid(a: u32, b: u16, c: u16, d: u64) -> Guid {
	Guid {
		data1: a,
		data2: b,
		data3: c,
		data4: d.to_be_bytes(),
	}
}

#[allow(non_upper_case_globals)]
pub const FOLDERID_UserProgramFilesCommon: &Guid =
	&Guid(0xBCBD3057, 0xCA5C, 0x4622, 0xB42D_BC56DB0AE516);

/// Get a path from a GUID.
// TODO: return a proper error.
pub fn known_folder(id: &Guid) -> Result<PathBuf, u32> {
	unsafe {
		let mut path_ptr = ptr::null_mut();
		let result = SHGetKnownFolderPath(id, KF_FLAG_CREATE, 0, &mut path_ptr);
		if result == 0 {
			let mut len = 0;
			let mut pos = path_ptr;
			while *pos != 0 {
				len += 1;
				pos = pos.add(1);
			}
			let path = String::from_utf16_lossy(slice::from_raw_parts(path_ptr, len));
			CoTaskMemFree(path_ptr.cast());
			Ok(path.into())
		} else {
			Err(result)
		}
	}
}

/// A registry key handle for reading a writing values.
pub struct RegKey {
	inner: usize,
}
impl RegKey {
	/// Open a registry key for reading and writing values.
	pub fn open(name: &str) -> Result<Self, io::Error> {
		let name: Vec<u16> = name.encode_utf16().chain(once(0)).collect();
		let mut handle: usize = 0;
		unsafe {
			let result = RegOpenKeyExW(
				HKEY_CURRENT_USER,
				name.as_ptr(),
				0,
				KEY_QUERY_VALUE | KEY_SET_VALUE,
				&mut handle,
			);
			if result != 0 {
				Err(io::Error::from_raw_os_error(result as _))
			} else {
				Ok(Self { inner: handle })
			}
		}
	}
	/// Get a value from this key.
	pub fn query_value(&self, name: &str) -> Result<String, io::Error> {
		let name: Vec<u16> = name.encode_utf16().chain(once(0)).collect();
		unsafe {
			let mut ty: u32 = 0;
			let mut buffer = vec![0; 512];
			let mut result;
			let mut len = (buffer.len() * 2) as _;
			loop {
				result = RegQueryValueExW(
					self.inner,
					name.as_ptr(),
					0,
					&mut ty,
					buffer.as_mut_ptr(),
					&mut len,
				);
				if result == ERROR_MORE_DATA {
					buffer.resize((len / 2) as _, 0);
				} else {
					break;
				}
			}
			if result != 0 {
				Err(io::Error::from_raw_os_error(result as _))
			} else {
				if ty != REG_SZ && ty != REG_EXPAND_SZ {
					panic!("Unexpected registry type");
				}
				Ok(String::from_utf16_lossy(&buffer[..(len / 2) as _]))
			}
		}
	}
	/// Write the value.
	pub fn set_value(&self, name: &str, value: &str) -> io::Result<()> {
		let name: Vec<u16> = name.encode_utf16().chain(once(0)).collect();
		let null = if value.ends_with("\0") { None } else { Some(0) };
		let value: Vec<u16> = value.encode_utf16().chain(null).collect();
		unsafe {
			let result = RegSetValueExW(
				self.inner,
				name.as_ptr(),
				0,
				REG_EXPAND_SZ,
				value.as_ptr(),
				(value.len() * 2) as _,
			);
			if result != 0 {
				Err(io::Error::from_raw_os_error(result as _))
			} else {
				Ok(())
			}
		}
	}
}
impl Drop for RegKey {
	fn drop(&mut self) {
		unsafe { RegCloseKey(self.inner) };
	}
}

/// Broadcast `WM_SETTINGCHANGE` to notify other applications that the
/// environment has changed.
pub fn broadcast_changes() -> io::Result<()> {
	unsafe {
		let result = SendMessageTimeoutA(
			HWND_BROADCAST,
			WM_SETTINGCHANGE,
			ptr::null(),
			"Environment\0".as_ptr(),
			SMTO_NOTIMEOUTIFNOTHUNG,
			5000,
			ptr::null_mut(),
		);
		if result != 0 {
			Ok(())
		} else {
			Err(io::Error::last_os_error())
		}
	}
}

const KF_FLAG_CREATE: u32 = 0x8000;
const HKEY_CURRENT_USER: usize = 0x80000001;
const KEY_QUERY_VALUE: u32 = 1;
const KEY_SET_VALUE: u32 = 2;
const REG_SZ: u32 = 1;
const REG_EXPAND_SZ: u32 = 2;
const ERROR_MORE_DATA: u32 = 234;
const HWND_BROADCAST: usize = 0xffff;
const WM_SETTINGCHANGE: u32 = 0x1A;
const SMTO_NOTIMEOUTIFNOTHUNG: u32 = 8;

#[link(name = "Advapi32")]
extern "system" {
	fn RegOpenKeyExW(
		hKey: usize,
		lpSubKey: *const u16,
		ulOptions: u32,
		samDesired: u32,
		phkResult: *mut usize,
	) -> u32;
	fn RegQueryValueExW(
		hKey: usize,
		lpValueName: *const u16,
		lpReserved: usize,
		lpType: *mut u32,
		lpData: *mut u16,
		lpcbData: *mut u32,
	) -> u32;
	fn RegSetValueExW(
		hKey: usize,
		lpValueName: *const u16,
		lpReserved: u32,
		dwType: u32,
		lpData: *const u16,
		lpcbData: u32,
	) -> u32;
	fn RegCloseKey(hKey: usize) -> u32;
}

#[link(name = "Ole32")]
extern "system" {
	fn CoTaskMemFree(pv: *mut u8);
}
#[link(name = "Shell32")]
extern "system" {
	fn SHGetKnownFolderPath(
		rfid: &Guid,
		dwFlags: u32,
		hToken: usize,
		ppszPath: *mut *mut u16,
	) -> u32;
}
#[link(name = "User32")]
extern "system" {
	fn SendMessageTimeoutA(
		hWnd: usize,
		Msg: u32,
		wparam: *const u8,
		lParam: *const u8,
		fuFlags: u32,
		uTimeout: u32,
		lpdwResult: *mut usize,
	) -> i32;
}
