/*!
Objective-C Runtime bindings and wrapper for Rust.

# Messaging objects

Objective-C objects can be messaged using the [`msg_send!`](macro.msg_send!.html) macro:

``` no_run
# #[macro_use] extern crate objc;
# use objc::runtime::{BOOL, Class, Object};
# fn main() {
# unsafe {
let cls = Class::get("NSObject").unwrap();
let obj: *mut Object = msg_send![cls, new];
let hash: usize = msg_send![obj, hash];
let is_kind: BOOL = msg_send![obj, isKindOfClass:cls];
// Even void methods must have their return type annotated
let _: () = msg_send![obj, release];
# }
# }
```

# Declaring classes

Objective-C classes can even be declared from Rust using the functionality of
the [`declare`](declare/index.html) module.

# Exceptions

By default, if the `msg_send!` macro causes an exception to be thrown, this
will unwind into Rust resulting in unsafe, undefined behavior.
However, this crate has an `"exception"` feature which, when enabled, wraps
each `msg_send!` in a `@try`/`@catch` and panics if an exception is caught,
preventing Objective-C from unwinding into Rust.
*/

#![crate_name = "objc"]
#![crate_type = "lib"]

#![warn(missing_docs)]

extern crate malloc_buf;
#[cfg(feature = "exception")]
extern crate objc_exception;

pub use encode::{Encode, EncodeArguments, Encoding};
pub use message::{Message, MessageArguments, MessageError};

pub use message::send_message as __send_message;
pub use message::send_super_message as __send_super_message;

#[macro_use]
mod macros;

pub mod runtime;
pub mod declare;
mod encode;
#[cfg(feature = "exception")]
mod exception;
#[cfg(feature = "exception")]
mod id;
mod message;

#[cfg(test)]
mod test_utils;

// The from_c_str and to_c_str methods are unfortunately needed because,
// prior to Rust 1.6, CStr uses a different definition of c_char than
// std::os::raw on arm64.
// TODO: remove this after most people are on Rust 1.6+

use std::ffi::CStr;
use std::os::raw::c_char;

unsafe fn from_c_str<'a>(ptr: *const c_char) -> &'a CStr {
    CStr::from_ptr(ptr as *const _)
}

fn to_c_str(s: &CStr) -> *const c_char {
    s.as_ptr() as *const _
}
