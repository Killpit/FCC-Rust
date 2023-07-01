pub fn run() {
    let arr: [char; 3] = ['中', '国', '人'];

    let slice: &[char] = &arr[..2];

    assert!(std::mem::size_of_val(&slice) == 16);

    println!("Success");
}