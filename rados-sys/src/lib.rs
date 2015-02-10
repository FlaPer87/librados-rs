#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(raw_pointer_derive)]

#![feature(libc)]

extern crate libc;

type __builtin_va_list = libc::c_void;

include!("ffi.rs");
