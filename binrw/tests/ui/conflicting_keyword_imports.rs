#![feature(generic_associated_types)]
use binrw::BinRead;

#[derive(BinRead)]
#[br(import(a: i32), import_raw(args: (i32, )))]
struct Foo;

fn main() {}
