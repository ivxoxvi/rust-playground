#[test]
fn test_utf8() {
    let hello = String::from("Здравствуйте");
    let s = &hello[0..4];
    println!("{s}");
}
