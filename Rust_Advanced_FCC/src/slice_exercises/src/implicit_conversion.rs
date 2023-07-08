pub fn run() {
    let mut s: String = String::from("hello world");

    let word: &str = first_word(&s);

    s.clear();

    println!("the first word is: {}", word);
}

fn first_word(s: &str) -> &str {
    &s[..1]
}