struct Val<T> {
    val: T,
}

impl<T> Val<T> {
    fn value(&self) -> &f64 {
        &self.val
    }
}

pub fn run() {
    let x: Val<f64> = Val{val: 3.0};
    let y: Val<String> = Val{val: "hello".to_string()};

    println!("{}, {}", x.value(), y.value());
}