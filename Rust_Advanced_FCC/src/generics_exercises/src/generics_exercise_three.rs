struct Point<T> {
    x: T,
    y: T,
}

pub fn run() {
    let p = Point{x: 5, y: "hello".to_string()};

    println!("Success");
}