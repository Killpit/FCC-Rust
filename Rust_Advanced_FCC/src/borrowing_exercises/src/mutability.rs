pub fn run() {
    let mut s : String = String::from("hello, ");

    borrow_object(&mut s);

    println!("Success")
}

fn borrow_object(s: &mut String) {}