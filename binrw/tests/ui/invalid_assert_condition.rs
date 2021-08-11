#![feature(generic_associated_types)]
use binrw::BinRead;

#[derive(BinRead)]
#[br(assert("wrong type"))]
struct Foo {
    a: i32,
}

fn main() {}
