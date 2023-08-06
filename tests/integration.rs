// Workaround for https://github.com/rust-lang/rust/issues/94348
extern crate shim_3ds;

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
#[should_panic]
fn it_fails() {
    assert_eq!(2 + 2, 5);
}
