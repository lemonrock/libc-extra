// This file is part of libc-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT. No part of libc-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of libc-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT.


use ::libc::c_ulong;


pub const SECUREBITS_DEFAULT: c_ulong = 0;
pub const SECURE_NOROOT: c_ulong = 0;
pub const SECURE_NOROOT_LOCKED: c_ulong = 1;
pub const SECURE_NO_SETUID_FIXUP: c_ulong = 2;
pub const SECURE_NO_SETUID_FIXUP_LOCKED: c_ulong = 3;
pub const SECURE_KEEP_CAPS: c_ulong = 4;
pub const SECURE_KEEP_CAPS_LOCKED: c_ulong = 5;
pub const SECURE_NO_CAP_AMBIENT_RAISE: c_ulong = 6;
pub const SECURE_NO_CAP_AMBIENT_RAISE_LOCKED: c_ulong = 7;
