// String

#[test]
fn test_utf8() {
    let hello = String::from("Здравствуйте");
    let s = &hello[0..4];
    println!("{s}");
}

// Ownership System

#[allow(dead_code)]
struct Inner;
#[allow(dead_code)]
struct Outer {
    inner: Inner,
}

#[cfg(false)]
#[test]
fn move_from_reference() {
    let o = Outer { inner: Inner };
    let i = (&o).inner; //❗️cannot move out of a shared reference
}

#[cfg(false)]
#[test]
fn borrow_mut_reference_from_reference() {
    let o = Outer { inner: Inner };
    let i = &mut (&o).inner; // ❗️cannot borrow data in a & reference as mutable
}

#[test]
fn borrow_mut_reference_from_mut_reference() {
    let mut o = Outer { inner: Inner };
    let _i = &mut (&mut o).inner; // 👍
}
