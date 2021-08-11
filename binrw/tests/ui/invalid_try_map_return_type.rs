#![feature(generic_associated_types)]
use binrw::BinRead;

#[derive(BinRead)]
struct Foo {
    #[br(try_map = |_| 0)]
    a: i32,
}

fn main() {}
