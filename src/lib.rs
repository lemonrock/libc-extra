// This file is part of libc-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT. No part of libc-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libc-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT.


#[cfg(any(target_os = "android", target_os = "linux"))] #[macro_use] extern crate cfg_if;
extern crate const_cstr_fork;
extern crate libc;


use ::libc::FILE;


#[cfg(any(target_os = "android", target_os = "linux"))] pub mod android_linux;


#[cfg(any(target_os = "android", target_os = "linux"))] pub use android_linux::*;


#[cfg(target_os = "freebsd")] pub mod freebsd;


#[cfg(target_os = "freebsd")] pub use freebsd::*;


pub mod ffi;


include!("stdin.rs");
include!("stdout.rs");
include!("stderr.rs");
