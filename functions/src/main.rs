fn main() {
    println!("implicit_return()            = {}", implicit_return());
    println!("does_not_return_last_value() = {}", does_not_return_last_value());
}

fn implicit_return() -> i8 {
    3
}

fn does_not_return_last_value() -> i8 {
    // because the final expression has a semicolon, 
    // Rust will not implicitly return it, and thus,
    // an error will be thrown. either the semicolon
    // is removed, or a return keyword must be added
    // in order to return the value
    4;
}
