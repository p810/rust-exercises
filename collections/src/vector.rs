// 
// vectors allow us to store multiple values (of one type) in an array-like
// structure that puts all the values next to each other in memory. vectors
// are useful for things like lists
// 
pub fn vectors_are_instantiated_with_associated_function_new_or_vec_macro() -> Vec<i8> {
    // 
    // when instantiating a vector ahead of time, before you have values for
    // it, you hint the type on the variable that stores it and call the associated
    // function new() 
    // 
    // note that if you want to push values onto a vector later on, it must be mutable
    // 
    let _v: Vec<i8> = Vec::new();

    // 
    // otherwise, when you want to instantiate a vector with some initial values,
    // you aren't required to manually specify the type because the compiler will
    // infer it - and you can also use the `vec![]` macro
    // 
    let mut v = vec![1, 2, 3];

    for i in 4 .. 6 {
        v.push(i);
    };

    v
}

pub fn reading_vectors_is_done_with_indexing_syntax_or_get(v: &Vec<i8>) -> i8 {
    // 
    // using index syntax, this is similar to reading an array in most other languages,
    // the main difference being that we need to use an ampersand because we're referencing
    // a value off of the vector
    // 
    // if we attempt to use this syntax to access a non-existent index, the program will panic
    // (Rust's word for "throwing an error/exception")
    // 
    let x = &v[0];

    // 
    // a safer way of accessing values is by using the `get()` method, which returns an `Option`
    // for the value that we want, so if there isn't a value at the specified index we simply get
    // back an instance of `None`
    // 
    let y = match v.get(1) {
        Some(number) => number,
        None => &0,
    };

    x + y
}

// 
// this code will result in a panic, because accessing the first element will return an immutable
// reference, and calling `push()` entails a mutable borrow - this is because pushing a new element
// onto the vector may cause memory to need to be reallocated, to make room for the next value if
// there presently isn't enough memory in the vector's address space to accomodate the new value
// 
// a case like this would cause the immutable reference `first` to point to deallocated memory, which
// would cause the program to error out
// 
pub fn updating_vector_while_holding_immutable_reference_is_not_allowed() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    v.push(6);
}

// 
// looping over vectors can be done either immutably or mutably - if we want to make changes to the
// elements in a vector we'll go for the latter
// 
pub fn looping_over_vectors_with_for(v: Vec<i8>) -> Vec<i8> {
    for i in &v {
        // here we don't do anything with the value, just print it
        println!("{}", i);
    };

    let mut w = v.clone();

    for i in &mut w {
        // 
        // but here, thanks to the mutable reference, we increment each val
        // 
        // the dereference operator, `*`, is required to acquire the value 
        // that exists in the reference 
        // 
        *i += 1;
    };

    w
}

// 
// enums can be used to store values of different types in a vector - if your variant
// takes a value of a certain type, and your vector takes any variant of your enum, then
// technically you're still storing the proper type, it just has a reference to the value
// that you need
// 
// this is demonstrated below with the `SpreadsheetCell` enum which has variants that
// describe the different kinds of data that might exist in a cell of a spreadsheet
// 
// to quote the Rust book:
// 
//> When writing a program, if you don't know the exhaustive set of types the program
//> will get at runtime to store in a vector, the enum technique won't work. Instead,
//> you can use a trait object (...)
// 
// traits are going to be covered later on
// 
pub enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn stores_different_types_via_an_enum() -> Vec<SpreadsheetCell> {
    vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(0.14),
        SpreadsheetCell::Text(String::from("Pi")),
    ]
}

// 
// the `get_mut()` method can be used to get a mutable reference for an element in a vector
// 
pub fn get_mut_returns_a_mutable_reference_for_an_element(v: &mut Vec<i8>) {
    if let Some(element) = v.get_mut(0) {
        *element += 10;
    }
}

// 
// slices are read only objects that are more commonly used in read only situations, since passing
// a vector means you're passing an object that can potentially be mutable
// 
// the syntax for slicing a vector is `&[type]`, and works for variable assignment as well
// 
pub fn slicing_can_be_used_to_get_a_read_only_array(v: &[i8]) {
    println!("v[0] = {}", v[0]);
}

// 
// the associated function `with_capacity()` can be used to specify how large a vector is expected to
// grow, which prevents the need to reallocate the memory for its elements when the length surpasses
// its capacity
// 
// it's recommended to use this constructor whenever possible to prevent unnecessary reallocation
// 
pub fn with_capacity_can_be_used_to_specify_vector_capacity() -> Vec<i8> {
    let mut vec = Vec::with_capacity(10);

    for i in  1 .. 10 {
        vec.push(i);
    }

    vec
}