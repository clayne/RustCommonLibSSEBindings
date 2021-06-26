extern crate bindgen;

use bindgen::RustTarget;
use std::env;
use std::path::PathBuf;

fn main() {
    std::env::set_var(
        "LIBCLANG_PATH",
        "D:\\Modding\\llvm-project\\llvm\\llvmTesting\\bin",
    );
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    //println!("cargo:rustc-link-lib=bz2");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.hpp");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.hpp")

        .clang_arg("-ID:\\SteamLibrary\\steamapps\\common\\Skyrim Special Edition\\src\\SpecalPlugin\\CommonLibSSE\\buildCl\\vcpkg_installed\\x64-windows-static-md\\include")
        .clang_arg("-ID:\\SteamLibrary\\steamapps\\common\\Skyrim Special Edition\\src\\SpecalPlugin\\CommonLibSSE\\include")
        .clang_arg("-std=c++20")
        .clang_arg("-fms-extensions")
        .clang_arg("-fms-compatibility")
        .fit_macro_constants(true)
        .layout_tests(false)
        .time_phases(true)
        .detect_include_paths(true)
        .respect_cxx_access_specs(true)
        .record_matches(true)
        .allowlist_function("*GetRTTI*")
        .allowlist_function("*SKSE*")
        .allowlist_function("*AsNode*")
        .allowlist_function("*GetFullName*")
        .allowlist_type("*SKSE*")
        .allowlist_var("*SKSE*")
        .allowlist_var("^RE_.*")
        .allowlist_type("^RE_.*")
        .allowlist_function("^RE_.*")
        .allowlist_function("^REL_.*")
        .allowlist_type("^REL_.*")
        .allowlist_var("^REL_.*")
        //.clang_arg("-ferror-limit=90000")
        //.clang_arg("-v")
        //.emit_clang_ast()
       //.emit_ir()
        //.emit_ir_graphviz("mydot.dot")
        .enable_cxx_namespaces()
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .rust_target(RustTarget::Nightly)
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
