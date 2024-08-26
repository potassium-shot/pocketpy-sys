#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
	use std::{
		ffi::{CStr, CString},
		ptr::null_mut,
	};

	use crate::*;

	const STRING_FILENAME: &CStr = c"<string>";

	#[test]
	fn hello_world() {
		unsafe {
			py_initialize();

			let hello_world: CString = CString::from(c"print('Hello World!')");

			if !py_exec(
				hello_world.as_ptr(),
				STRING_FILENAME.as_ptr(),
				py_CompileMode_EXEC_MODE,
				null_mut(),
			) {
				py_printexc();
				panic!("Error in py_exec");
			}

			py_finalize();
		}
	}
}
