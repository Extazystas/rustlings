// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let a = "This is a long enough sting to contain 100 elements. Or less. A bit more chars needed. May be now its 100.";

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("{} elements is not the right amount for an array.", a.len());
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed")
    }
}
