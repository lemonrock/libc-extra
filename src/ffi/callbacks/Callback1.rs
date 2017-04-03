// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


// Making this Sized prevents the use of fat pointers in callFromC
pub trait Callback1<R: Sized> : Sized
{
	#[inline(always)]
	fn call(&self) -> R;
	
	#[inline(always)]
	unsafe extern "C" fn callFromC(arg1: *mut c_void) -> R
	{
		let us: &mut Self = &mut *(arg1 as *mut Self);
		let result = us.call();
		forget(us);
		result
	}
	
	#[inline(always)]
	fn asFunctionPointer() -> Option<unsafe extern "C" fn(arg1: *mut c_void) -> R>
	{
		Some(Self::callFromC)
	}
	
	#[allow(trivial_casts)]
	#[inline(always)]
	fn asFunctionArgument(&self) -> *mut c_void
	{
		self as *const _ as *const c_void as *mut c_void
	}
}
