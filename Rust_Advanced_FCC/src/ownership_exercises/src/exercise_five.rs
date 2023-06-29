pub fn exercise_five() {
    let x: (i32, i32, (), String) = (1, 2, (), "hello".to_string());
    let y: (i32, i32, (), String) = x.clone();
    println!("{:?}, {:?}", x, y);
}