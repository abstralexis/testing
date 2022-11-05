struct Strings<'a> {
    string: String,
    borrowstr: &'a str,
}
impl<'a> Strings<'a> {
    pub fn new(string: String, borrowstr: &'a str) -> Self {
        Strings { string: string, borrowstr: borrowstr }
    }

    pub fn get_string(&self) -> String {
        return self.string.to_owned() + self.borrowstr;
    }
}

fn main() { 
    println!("Hello, world!");
}
