#[cfg(feature="gnustep_runtime")]
extern crate gcc;
#[cfg(feature="gnustep_runtime")]
use std::path::PathBuf;


#[cfg(not(feature="gnustep_runtime"))]
fn compile() {
}

#[cfg(feature="gnustep_runtime")]
fn compile() {
    gcc::Config::new().flag("-lobjc")
                      .flag("-fobjc-runtime=gnustep-1.8")
                      .flag("-fno-objc-legacy-dispatch")
                      .file("extern/NSObject.m")
                      .cargo_metadata(false)
                      .compile("libNSObject.a");
    let path = ::std::env::var_os("OUT_DIR").map(PathBuf::from).unwrap();
    println!("cargo:rustc-link-search=native={}", path.display()); 
}
fn main() {
    compile();
}
