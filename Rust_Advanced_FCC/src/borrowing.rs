/*
Borrowing:

-Way of temporarily access data without taking ownership of it.
-When borrowing, you're taking a reference (pointer) to the data, not the data itself.
-Prevention of dangling pointers and data races 
-Data can be borrowed immutably and mutably
-There are certain rules when borrowing which we have to comply with, otherwise, the program won't compile */

/*
Rules of References

-At any given time, you can have either one mutable reference or any number of immutable references.
-References must always be valid
*/

pub fn run() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}