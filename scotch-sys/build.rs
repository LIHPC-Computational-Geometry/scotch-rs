use std::env;
use std::ffi::OsString;
use std::path::PathBuf;
use std::process;

fn main() {
    if env::var_os("DOCS_RS").is_some() {
        // Do not generate bindings when run in docs.rs.
        return;
    }

    println!("cargo:rustc-link-lib=scotch");
    println!("cargo:rustc-link-lib=scotcherr");
    println!("cargo:rerun-if-changed=wrapper.h");

    let mut cpath = env::var_os("CPATH").unwrap_or_else(OsString::new);
    if !cpath.is_empty() {
        cpath.push(":");
    }
    cpath.push("/usr/include/scotch");
    env::set_var("CPATH", cpath);

    let bindings = bindgen::builder()
        .header("wrapper.h")
        .allowlist_function("SCOTCH_.*")
        .allowlist_type("SCOTCH_.*")
        .allowlist_var("SCOTCH_.*")
        .allowlist_function("fdopen") // provide FILE*
        .allowlist_function("fclose")
        .opaque_type("FILE")
        .generate()
        .unwrap_or_else(|err| {
            eprintln!("Failed to generate bindings to Scotch: {err}");
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
