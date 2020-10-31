fn main() {
    println!("Hello, world!");
}

// 
// vectors allow us to store multiple values (of one type) in an array-like
// structure that puts all the values next to each other in memory. vectors
// are useful for things like lists
// 
mod vector {

    pub fn vectors_are_instantiated_with_associated_function_new_or_vec_macro() -> Vec<i8> {
        // 
        // when instantiating a vector ahead of time, before you have values for
        // it, you hint the type on the variable that stores it and call the associated
        // function new() 
        // 
        // note that if you want to push values onto a vector later on, it must be mutable
        // 
        let v: Vec<i8> = Vec::new();

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

    pub fn reading_vectors_is_done_with_indexing_syntax_or_get(v: Vec<i8>) -> i8 {
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
}