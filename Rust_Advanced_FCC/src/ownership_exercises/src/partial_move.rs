/*
Within the destructuring of a single variable, both by-move and by-reference pattern bindings can be used at the same
time. Doing this will result in a partial move of the variable, which means that parts of the variable will be moved 
while other parts stay. In such a case, the parent variable cannot be used afterwards as a whole, however the parts that
are only referenced (and not moved) can still be used */

pub fn run() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person: Person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    //'name' is moved out of person, but 'age' is referenced
    let Person {name, ref age} = person;

    println!("The person's age is {}", age);
    println!("The person's name is {}", name);
}