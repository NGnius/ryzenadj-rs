use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search=ryzenadj/build/");

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=ryzenadj");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=src/ryzenadj.h");

    // Run `clang` to compile the `hello.c` file into a `hello.o` object file.
    // Unwrap if it is not possible to spawn the process.
    if !std::process::Command::new("cmake")
        .arg("-S").arg("./ryzenadj")
        .arg("-B").arg(env::var("OUT_DIR").unwrap())
        .arg("-DCMAKE_BUILD_TYPE=Release")
        //.arg("ryzenadj/")
        .output()
        .expect("could not cmake ryzenadj")
        .status
        .success()
    {
        // Panic if the command was not successful.
        panic!("could not compile object file");
    }

    // Run `ar` to generate the `libhello.a` file from the `hello.o` file.
    // Unwrap if it is not possible to spawn the process.
    if !std::process::Command::new("make")
        .arg("-C").arg(env::var("OUT_DIR").unwrap())
        .output()
        .expect("could not make ryzenadj")
        .status
        .success()
    {
        // Panic if the command was not successful.
        panic!("could not compile ryzenadj");
    }

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("./src/ryzenadj.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
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
