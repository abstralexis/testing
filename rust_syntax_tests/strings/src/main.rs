struct Strings<'a> {
    _string: String,
    _borrowstr: &'a str,
}
impl<'a> Strings<'a> {
    pub fn new(string: String, borrowstr: &'a str) -> Self {
        Strings { _string: string, _borrowstr: borrowstr }
    }

    pub fn get_string(&self) -> String {
        return self._string.to_owned() + self._borrowstr;
    }
}

fn main() { 
    let strings: Strings = Strings::new(String::from("Hello "), "World!");
    let strings_string: String = strings.get_string();
    println!("{}", strings_string);
}
