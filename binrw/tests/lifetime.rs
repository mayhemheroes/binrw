#![feature(generic_associated_types)]
use binrw::{derive_binread, io::Cursor, BinReaderExt};
use core::marker::PhantomData;

#[test]
fn derive_reference() {
    struct HasLifetime<'unused>(PhantomData<&'unused ()>);

    #[derive_binread]
    #[derive(PartialEq, Debug)]
    struct Test {
        #[br(calc = 3)]
        x: u32,

        #[br(args(&x, &x, &x, &HasLifetime(PhantomData)))]
        z: PassReference,
    }

    #[derive_binread]
    #[derive(PartialEq, Debug)]
    #[br(import(val: &u32, val2: &'_ u32, val3: &'this u32, _val4: &HasLifetime<'_>))]
    struct PassReference {
        #[br(calc = *val + *val2 + *val3)]
        x_copy: u32,
    }

    let mut cursor = Cursor::new(&[]);
    let test: Test = cursor.read_le().unwrap();

    assert_eq!(
        test,
        Test {
            x: 3,
            z: PassReference { x_copy: 9 }
        }
    );
}
