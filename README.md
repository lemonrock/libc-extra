[](This file is part of libc-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT. No part of libc-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.)
[](Copyright © 2016 The developers of libc-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT.)

# libc-extra

[![Clippy Linting Result](https://clippy.bashy.io/github/lemonrock/libc-extra/master/badge.svg?style=plastic)](https://clippy.bashy.io/github/lemonrock/libc-extra/master/log) [![](https://img.shields.io/badge/Code%20Style-rustfmt-brightgreen.svg?style=plastic)](https://github.com/rust-lang-nursery/rustfmt#configuring-rustfmt)

[libc-extra] is a rust crate that supplies additional bits and bobs found in libc libraries that either hasn't been submitted to Rust's official libc yet, or has been rejected or requires too much work to integrate. Some things in here are uncertain `asm_generic` and `linux` aren't reallty part of libc. Mostly it is intended that code in here should change frequently, and migration when something is officially accepted by the libc crate should require breaking change replacement of `::libc_extra::` for `::libc::`.


## Licensing

The license for this project is MIT.

[libc-extra]: https://github.com/lemonrock/libc-extra "libc-extra GitHub page"
