// Import (via `use`) the `fmt` module to make it available.
use std::fmt;

/*
* These are multiline comments
* They span multiple lines
*/

// This is a single line comment

// All types can derive (automatically create) the fmt::Debug implementation.
#[derive(Debug)]
struct Structure(i32);
// This is not true for fmt::Display which must be manually implemented.

#[derive(Debug)]
struct Deep(Structure);

// To use the `{}` marker, the trait `fmt::Display` must be implemented for the type
impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// These comments generate library docs for the following item
fn main() {
    //! These comments generate library docs for the enclosing item
    println!("Hello, world!");

    /*
    Printing is handled by a series of macros defined in std::fmt some of which include:
        format!: write formatted text to String
        print!: same as format! but the text is printed to the console (io::stdout).
        println!: same as print! but a newline is appended.
        eprint!: same as print! but the text is printed to the standard error (io::stderr).
        eprintln!: same as eprint! but a newline is appended.
    */

    eprintln!("This is an standard error");

    println!("My name is {} and I am {} years old", "Gianluca", 28);
    println!(
        "{lang} is my favourite language as a {job}.",
        lang = "Rust",
        job = "Software engineer",
    );
    // Only types that implement fmt::Display can be formatted with `{}`.
    println!(
        "My favourite number is {n}, which is {n:b} in binary",
        n = 27
    );
    // Print pi to 3 decimal places
    let pi = 3.141592;
    println!("I also like the number {:.3}", pi);
    // fmt::Debug: Uses the {:?} marker. Format text for debugging purposes.
    println!("My name is {:?} and I am {:?} years old", "Gianluca", 28);

    println!("Now debug print Structure(3) as {:?}", Structure(3));
    println!(
        "Now debug print Deep(Structure(3)) as {:?}",
        Deep(Structure(3))
    );
    println!(
        "Or pretty print Deep(Structure(3)) as {:#?}",
        Deep(Structure(3))
    );
    println!("Or use our own Display impl: {}", Structure(3));
}
