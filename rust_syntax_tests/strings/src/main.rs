/* 
Demonstrating some quirks with str and String.
I don't understand why it is like  right now, 
so I did it to figure out what is going on.
*/

// Use a struct for this example
struct Strings<'a> {        // The lifetime param of the struct
    _string: String,        // A String is a smart pointer on the heap
    _borrowstr: &'a str,    // A str is an unknown size slice of some string so
                            // we must borrow and explicitly handle the lifetime
}
impl<'a> Strings<'a> {      // Here we need to make sure the str is still alive
    pub fn new(string: String, borrowstr: &'a str) -> Self {
        Strings { _string: string, _borrowstr: borrowstr }
    }

    pub fn get_string(&self) -> String {
        // Get an owned version of string and implicitly deref borrowstr
        return self._string.to_owned() + self._borrowstr;
    }
}

fn main() { 
    // Demonstration
    let strings: Strings = Strings::new(String::from("Hello "), "World!");
    let strings_string: String = strings.get_string();
    println!("{}", strings_string);
}

/*
I still don't understand it, but I understand it more now.
*/