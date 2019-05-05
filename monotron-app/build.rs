use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=monotron-app.ld");
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let linker_script = include_bytes!("monotron-app.ld");
    let mut f = File::create(out.join("monotron-app.ld")).unwrap();
    f.write_all(linker_script).unwrap();
    println!("cargo:rustc-link-search={}", out.display());
}
