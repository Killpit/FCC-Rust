fn sum<T>(a: T, b: T) -> T {
    a + b
}

pub fn run() {
    assert_eq!(5, sum(2i8, 3i8));
    assert_eq!(50, sum(20, 30));
    assert_eq!(2.46, sum(1.23, 1.23));

    println!("Success");
}