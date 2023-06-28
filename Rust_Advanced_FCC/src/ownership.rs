/*

-Rust's ownership is unique and sets it apart from other programming languages
-Set of rules that govern memory management
-Rules are enforced at compile time
-If any of the rules are violated, the program won't compile

*/ 

/*

Scope:

-Range within a program for which an item is valid

Global Scope:

-Accessible throughout the entire program

Local Scope:

-Accessible only within particular function or block of code
-Not accessible outside of that function or block

*/ 

/* 

Memory

-Component in a computer to store data and instructions for the processor to execute
-Random Access Memory (RAM) is volatile, when power turned off all contents are lost
-Two types of regions in RAM used by a program at runtime: Stack memory and heap memory

*/

/*

Preventing issues:



 */

pub fn ownership() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); //s's value moves into the function ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); //x would move into the function, but i32 is Copy, so it's okay to still use x afterward
} //Here, x goes out of scope, then s. But because s's value was moved, nothing special happens

fn takes_ownership(some_string: String) { //some_string comes into scope
    println!("{}", some_string);
} //Here, some_string goes out of scope and 'drop' is called. The backing memory is freed

fn makes_copy(some_integer: i32) { //some_integer comes into scope
    println!("{}", some_integer);
} //Here, some_integer goes out of scope. Nothing special happens