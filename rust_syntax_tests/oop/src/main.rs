/*
This is for demonstrating Rust syntax that can be used for
OOP. It is different to many other things and I used to not like
it but I am starting to warm up to it.
*/

// This allows dead_code. Stops a warning for not using things.
// Don't do this in production, this is just because I can't be bothered
// To make an example for enums.
#![allow(dead_code)]

// An enum is like a type that can have different states.
enum Options {
    Yes,
    No
}

// Make a structure for a person
// Struct names are in CamelCase.
struct Person {
    // Define some properties for the person
    name: String,   // Properties are private by default
    pub age: u8,    // So we can make them public with the pub keyword.
}
// Implement some methods for the person
impl Person {
    // This creates a new instance of the struct.
    // In rust, it is convention to use struct::new() as a constructor.
    fn new(name: String, age: u8) -> Self {
        // Implicitly returns a new instance of Person.
        Person { 
            name: name,
            age: age,
        }
    }

    // Here, we make a getter function for name as it's private.
    // First, we must borrow self to access the variables.
    // &self is equal to self: &Self.
    fn get_name(&self) -> String {
        // We must clone the String as it does not implement the Copy trait.
        let name_cpy = self.name.clone();
        return name_cpy;    // Here, an explicit return is needed
    }
}

fn main() {
    // Make a person with struct literal syntax. 
    let person: Person = Person {
        name: String::from("Bob"),
        age: 36,
    };

    // Make a new person with the constructor we made.
    let new_person: Person = Person::new(String::from("Dave"), 45);

    // Get the names and ages
    let name: String = person.get_name();   // Use getter 
    let age: u8 = person.age;               // Get public variable

    let new_name: String = new_person.get_name();
    let new_age: u8 = new_person.age;

    // Print what we got
    println!("{} is {}", name, age);
    println!("{} is {}", new_name, new_age);
}
