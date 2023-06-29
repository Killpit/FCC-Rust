pub fn boxed_integer() {
    let x: Box<i32> = Box::new(5);

    let mut y: Box<i32> = Box::new(1);

    *y = 4;

    assert_eq!(*x, 5);

    println!("Success");
}