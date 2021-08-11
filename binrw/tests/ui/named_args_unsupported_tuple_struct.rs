#![feature(generic_associated_types)]
use binrw::BinrwNamedArgs;

#[derive(BinrwNamedArgs)]
struct Tuple(u32, String);

fn main() {}
