pub fn run() {
    lat mut s: String = String::from("hello, ");

    let r1: &mut String = &mut s;
    r1.push_str("world");
    
    let r2: &mut String = &mut s;
    r2.push_str("!");

    println!("{}", r1);
}
