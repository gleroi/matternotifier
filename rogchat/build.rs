use std::env;

fn main() {
    let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    println!("cargo:rustc-link-search={}\\libs", project_dir);

    pkg_config::Config::new().probe("gtk+-3.0").unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}
