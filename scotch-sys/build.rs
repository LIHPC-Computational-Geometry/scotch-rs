use std::env;
use std::path::PathBuf;
use std::process;

fn main() {
    println!("cargo:rustc-link-lib=scotch");

    // Take into account $CFLAGS and $LDFLAGS
    let cflags = env::var("CFLAGS").unwrap_or_default();
    //let ldflags = env::var("LDFLAGS").unwrap_or_default();
    let ldflags = String::new();
    let clang_args = cflags.split(' ').chain(ldflags.split(' '));

    let bindings = bindgen::builder()
        .clang_args(clang_args)
        .header("wrapper.h")
        .generate()
        .unwrap_or_else(|()| {
            eprintln!("Failed to generate bindings to Scotch");
            process::exit(1);
        });

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings.write_to_file(&out_path).unwrap_or_else(|err| {
        eprintln!(
            "Failed to write Scotch bindings to {:?}: {}",
            out_path.display(),
            err,
        );
        process::exit(1);
    });
}
