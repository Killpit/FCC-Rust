pub fn run() {
    let mut s = String::from("hello, ");

    borrow_objects(&s);

    s.push_str("world");

    println!("Success");
}
fn borrow_objects(s. &String) {}