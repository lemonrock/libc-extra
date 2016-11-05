// This file is part of libc-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT. No part of libc-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libc-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct cmsghdr
{
	pub cmsg_len: socklen_t,
	pub __pad1: c_int,
	pub cmsg_level: c_int,
	pub cmsg_type: c_int,
}

impl Default for cmsghdr
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
