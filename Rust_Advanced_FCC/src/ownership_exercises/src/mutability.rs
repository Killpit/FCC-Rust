pub fn mutability() {
    let s = String::from("hello, ");

    let mut s1 = s;

    s1.push_str("world");

    println!("Success!");
}