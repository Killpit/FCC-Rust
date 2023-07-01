pub fn run() {
    let s: String = String::from("hello");

    let slice1: &str = &s[0..2];
    let slice2 = &s[3..4];

    assert_eq!(slice1, slice2);

    println!("Success");
}
