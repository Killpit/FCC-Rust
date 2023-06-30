pub fn run() {
    let x: i32 = 5;
    let y: &i32 = &x;

    assert_eq!(5, *y);

    println!("Success");
}
