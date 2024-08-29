use std::{env, path::PathBuf};

fn main() {
	println!("cargo:rerun-if-changed=wrapper.h");

	let target = env::var("TARGET").unwrap();

	// Generated the bindings
	let bindings = bindgen::Builder::default()
		.header("wrapper.h")
		.parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
		.allowlist_item("py_.*|_py_.*|free")
		.clang_arg("-Ivendor/pocketpy/include")
		.generate()
		.expect("failed to generate bindings with Bindgen");

	// Write the bindings
	let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
	bindings
		.write_to_file(out_path.join("bindings.rs"))
		.expect("failed to write generated bindings");

	// We have to force disable IPO/LTO otherwise linker will crash
	const CMAKE_LISTS_PATH: &str = "./vendor/pocketpy/CMakeLists.txt";

	let original_cmake_lists_contents = std::fs::read_to_string(CMAKE_LISTS_PATH).unwrap();

	std::fs::write(
		CMAKE_LISTS_PATH,
		original_cmake_lists_contents.replace(
			"set(CMAKE_INTERPROCEDURAL_OPTIMIZATION TRUE)",
			"set(CMAKE_INTERPROCEDURAL_OPTIMIZATION FALSE)",
		),
	)
	.unwrap();

	// Build pocketpy as static lib
	let built = cmake::Config::new("vendor/pocketpy")
		.define("PK_BUILD_SHARED_LIB", "OFF")
		.define("PK_BUILD_STATIC_LIB", "ON")
		.build_target("pocketpy")
		.build();

	// Add the compiled lib to the library search path of rustc
	if target.contains("windows") {
		// todo: test and change accordingly
		println!("cargo:rustc-link-search=native={}/build", built.display());
	} else {
		println!("cargo:rustc-link-search=native={}/build", built.display());
	}

	println!("cargo:rustc-link-lib=static=pocketpy");

	// Restore the original CMakeLists.txt
	std::fs::write(CMAKE_LISTS_PATH, original_cmake_lists_contents).unwrap();
}
