// This file is part of libc-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT. No part of libc-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libc-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT.


use ::libc::c_char;
use ::libc::c_int;
use ::libc::c_void;
use ::libc::FILE;
use ::libc::off_t;
use ::libc::size_t;
use ::libc::ssize_t;


#[link(name = "c")]
extern
{
	pub fn fopencookie(args: *mut c_void, path: *const c_char, functions: cookie_io_functions_t) -> *mut FILE;
}


pub type cookie_read_function_t = unsafe extern "C" fn(cookie: *mut c_void, data: *mut c_char, length: size_t) -> ssize_t;


pub type cookie_write_function_t = unsafe extern "C" fn(cookie: *mut c_void, data: *const c_char, length: size_t) -> ssize_t;


pub type cookie_seek_function_t = unsafe extern "C" fn(cookie: *mut c_void, offset: *mut off_t, c: c_int) -> c_int;


pub type cookie_close_function_t = unsafe extern "C" fn(cookie: *mut c_void) -> c_int;


//noinspection SpellCheckingInspection
/// Used by `fopencookie`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct cookie_io_functions_t
{
	/// Read function pointer.
	pub read: cookie_read_function_t,
	
	/// Write function pointer.
	pub write: cookie_write_function_t,
	
	/// Seek function pointer.
	pub seek: cookie_seek_function_t,
	
	/// Close function pointer.
	pub close: cookie_close_function_t,
}

impl Default for cookie_io_functions_t
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			read: Self::default_read,
			write: Self::default_write,
			seek: Self::default_seek,
			close: Self::default_close,
		}
	}
}

impl cookie_io_functions_t
{
	/// No-op cookie read function.
	pub unsafe extern "C" fn default_read(_cookier: *mut c_void, _data: *mut c_char, _length: size_t) -> ssize_t
	{
		0
	}
	
	/// No-op cookie write function.
	pub unsafe extern "C" fn default_write(_cookier: *mut c_void, _data: *const c_char, _length: size_t) -> ssize_t
	{
		0
	}
	
	/// No-op cookie seek function.
	pub unsafe extern "C" fn default_seek(_cookier: *mut c_void, _offset: *mut off_t, _c: c_int) -> c_int
	{
		0
	}
	
	/// No-op cookie close function.
	pub unsafe extern "C" fn default_close(_cookier: *mut c_void) -> c_int
	{
		0
	}
}
