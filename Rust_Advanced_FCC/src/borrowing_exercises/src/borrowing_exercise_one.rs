pub fn run() {
    let x: i32 = 5;
    let p: &i32 = &x;

    println!("the memory address of x is {:p}", p);
}