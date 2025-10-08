#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
fn main() {
    {
        ::std::io::_print(format_args!("Macro sample in rust\n"));
    };
    let vec = <[_]>::into_vec(::alloc::boxed::box_new([1, 2, 3, 4, 5]));
    {
        ::std::io::_print(format_args!("{0:?}\n", vec));
    };
}
