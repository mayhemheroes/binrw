#![feature(generic_associated_types)]
use binrw::BinRead;

#[derive(BinRead)]
enum UnitEnum {
    A,
}

fn main() {}
