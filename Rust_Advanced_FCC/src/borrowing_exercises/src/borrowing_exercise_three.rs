pub fn run() {
    let mut s: String = String::from("hello, ");

    borrow_objects(s);

    println!("Success!");
}

fn borrow_objects(s: String) {}