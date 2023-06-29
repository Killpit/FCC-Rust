pub fn exercise_three() {
    let s: String = give_ownership();
    println!("{}", s);
}

fn give_ownership() -> String {
    let s: String = String::from("hello, world");
    let _s = s.as_bytes();
    s
}