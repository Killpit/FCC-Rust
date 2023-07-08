pub fn run() {
    let mut s: String = String::from("hello world");

    let word: &str = first_word(&s);
    println!("the first word is: {}", word);

    s.clear();
}

fn first_word(s: &str) -> &str {
    &s[..1]
}