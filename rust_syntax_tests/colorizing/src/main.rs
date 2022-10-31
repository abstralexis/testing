/*
This is going to use the colored crate to create
coloured text in the terminal. This could be used for
making cooler looking CLI applications or games!
*/

// The use keyword in Rust is like import in Python.
use colored::Colorize;

fn main() {
    // Use the methods given to us by colored::Colorize on the string literals
    let bold = "Bold".bold();
    let green = "Green".green();
    let purple_italic = "Purple Italic".purple().italic();

    // Print the formatted text to the console
    println!("{} {} {}", bold, green, purple_italic);
}

/*
What is annoying is that programming languages use American
spelling for things, so it may be easy to mess up as a British
person and type colour or colourise. Will take some getting used to.
*/