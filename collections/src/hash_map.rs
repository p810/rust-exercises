#![allow(unused)]

// 
// hash maps are analogous to associative arrays in PHP or objects in JavaScript - they are a
// key value store that use a hashing algorithm under the hood to map keys to their elements
// 
// unlike vectors and strings, the hash map's object must be imported from the standard library - 
// the book says something about a prelude, which I assume will be covered later, but I'm gathering
// that this is an assortment of types and objects that are automatically brought into scope from
// the standard lib:
// 
//> Note that we first need to `use` the `HashMap` from the collections portion of the standard
//> library. Of our three common collections [in this chapter], this one is the least often used,
//> so it's not included in the features brought into scope automatically in the prelude.
// 
use std::collections::HashMap;

pub fn constructing_hash_map_with_associated_function() -> HashMap<String, String> {
    let mut x = HashMap::new();

    x.insert(String::from("foo"), String::from("bar"));

    x
}

// 
// it's possible to use `collect()` to turn a couple of iterators into a hash map, with the following
// caveats:
// 
// 1. the type must be declared - it's declared here as the return type, and cannot (to my knowledge)
//    be inferred by the compiler in variable assignment, so it has to be declared there too - but
//    underscores can be used in place of the generic types, if necessary, and that can be inferred
// 2. the iterator must yield tuples that contain one key and one value, to be used to create the
//    collection
// 
// an alternate way of declaring the type of the collection is with the turbofish syntax:
// 
// ```rust
// keys.into_iter()
//     .zip(vals.into_iter())
//     .collect::<HashMap<_, _>>();
// ```
// 
pub fn constructing_hash_map_from_collect() -> HashMap<String, String> {
    let keys = vec![String::from("a"), String::from("c")]; 
    let vals = vec![String::from("b"), String::from("d")];

    keys.into_iter()           // acquire an iterator for the keys
        .zip(vals.into_iter()) // call zip() to use the above iter as keys, and the given iter as vals
        .collect()             // call collect() to assemble everything into a hash map
}

// 
// types that implement the `Copy` trait will be copied into a hash map (so I think this includes
// types that are stored on the stack? need to double check, could be more than just that), while
// those that don't will be moved into it, transferring ownership to the hash map
// 
pub fn owned_types_will_be_moved_into_a_hash_map() {
    let a = 1;
    let b = 2;
    let c = 3;

    let d: Vec<(&str, u8)> = vec![("a", a), ("b", b), ("c", c)];

    d.into_iter().collect::<HashMap<&str, u8>>();

    // we can still use variables a - c because their values are
    // copied into the vector & hash map
    println!("a = {}", a);
    println!("b = {}", b);

    let e = String::from("f");
    let mut f = HashMap::new();

    f.insert("e", e);

    // the following line would cause a panic, because we no longer own e
    // println!("e = {}", e);
}