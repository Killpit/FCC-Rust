struct Point<T, U> {
    x: T,
    y: U,
}

pub fn run() {
    let p = Point{x: 5, y: "hello".to_string()};

    println!("Success");
}