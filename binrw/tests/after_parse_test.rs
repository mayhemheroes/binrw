#![feature(generic_associated_types)]
use binrw::{io::Cursor, BinRead, BinReaderExt, FilePtr8};

#[test]
#[allow(non_snake_case)]
fn BinReaderExt_calls_after_parse() {
    let test: FilePtr8<u8> = Cursor::new([0x01, 0xFF]).read_be().unwrap();

    assert_eq!(*test, 0xFF);
}

#[derive(BinRead)]
struct Try<BR>(#[br(try)] Option<BR>)
    where for<'a> BR: BinRead<Args<'a> = ()>;

#[test]
fn try_calls_after_parse() {
    let test: Try<FilePtr8<u8>> = Cursor::new([0x01, 0xFF]).read_be().unwrap();

    assert_eq!(*test.0.unwrap(), 0xFF)
}
