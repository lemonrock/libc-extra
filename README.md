# libc-extra


[libc-extra] is a rust crate that supplies additional bits and bobs found in libc libraries that either hasn't been submitted to Rust's official libc crate yet, or has been rejected or requires too much work to integrate. Mostly it is intended that code in here should change frequently, and migration when something is officially accepted by the libc crate should require breaking change replacement of `::libc_extra::` for `::libc::`.

Some modules in here, such as `android_linux/asm_generic` (which contains constants originally sourced from the musl libc, but relevant to Linux syscalls) and `ffi` are only peripherally related to `libc`.


## Licensing

The license for this project is MIT.

[libc-extra]: https://github.com/lemonrock/libc-extra "libc-extra GitHub page"
