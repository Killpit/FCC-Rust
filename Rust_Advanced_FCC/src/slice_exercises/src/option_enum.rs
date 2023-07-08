/*
-Option is an enum that represents a value that may or may not be present.

-Known in other languages as "null", referring to the absence of value

-Used to handle cases where a function or method might fail to return a value
*/

pub fn run() {
    let five: Option<i32> = Option::Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    if let __ = six {
        println!("{}", n);

        println!("Success");
    }

    panic!("NEVER LET THIS RUN! ");
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}