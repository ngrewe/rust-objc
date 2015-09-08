#[cfg(feature="gnustep_runtime")]
extern crate gcc;

#[cfg(not(feature="gnustep_runtime"))]
fn compile() {
}

#[cfg(feature="gnustep_runtime")]
fn compile() {
    gcc::Config::new().flag("-lobjc")
                      .flag("-fobjc-runtime=gnustep-1.8")
                      .flag("-fno-objc-legacy-dispatch")
                      .file("extern/NSObject.m")
                      .link(false)
                      .compile("libNSObject.a");
}
fn main() {
    compile();
}
