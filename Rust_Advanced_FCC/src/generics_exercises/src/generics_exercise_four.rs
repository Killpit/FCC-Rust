struct Val<T> {
    val: T,
}

impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

pub fn run() {
    let x = Val{val: 3.0};
    let y = Val{val: "hello".to_string()};
    println!("{}, {}", x.value(), y.value());
}