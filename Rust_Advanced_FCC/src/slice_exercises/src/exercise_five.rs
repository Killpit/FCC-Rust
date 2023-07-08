pub fn run() {
    let s: &str = "你好，世界";

    let slice: &str = &s[0..3];

    assert!(slice == "你");

    println!("Success");
}