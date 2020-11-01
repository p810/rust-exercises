// 
// the `String` type shares many functions with other collection types, like vectors
// 
pub fn string_type_shares_methods_with_other_collection_types() -> String {
    String::new()
}

// 
// any type that implements the `Display` trait, like the core `str` type does, will have
// the `to_string()` method from which we can get a `String`
//
pub fn types_implementing_display_can_be_represented_as_strings() -> String {
    // this is equivalent to calling `String::from()`
    "Hello, world!".to_string()
}

// 
// strings can grow and mutate, and there are a handful of ways to do this:
// 
// - the `+` operator
// - the `format!()` macro
// - methods like `String::push()` and `String::push_str()`
//
pub fn strings_can_grow_and_change(s: &mut String) -> String {
    // `push_str()` takes a string slice to avoid transferring ownership of the param's value
    let greeting = "Hello";
    s.push_str(greeting);

    // `push()` allows appending a single character (`char`) which requires single quotes
    s.push(',');

    // `format!()` allows us to use interpolation similar to `println!()`
    format!("{} world!", s)
}

// 
// the logic behind the `+` operator is a little different from the above, and it will result in
// loss of ownership over the left hand argument due to how it's implemented
// 
// `+` resolves to `add()`, a function that's implemented in the standard lib (I assume via a trait)
// for string types:
// 
// ```rust
// fn add<T>(self, s: &T) -> T {/* ... */}
// ```
// 
// Rust will use the left hand argument as `self`, which moves it into `add()`, and then append the referenced
// contents of the right hand argument to it (after coercion, if necessary - basically converting a `&String`
// into `&str`, since two `String` values can't be added together - this is called deref coercion but hasn't
// been covered yet)
//
pub fn concatenation_with_plus_operator_returns_new_string(s: String) -> String {
    s + &String::from("!")
}