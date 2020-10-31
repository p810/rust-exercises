mod bar;
mod bam;

fn main() {
    //|                                                          |//
    //| we could also just do this:                              |//
    //|                                                          |//
    //|  >> use library::my_library                              |//
    //|                                                          |//
    //| and that would allow us to refer to any entity belonging |//
    //| to that namespace like so:                               |// 
    //|                                                          |//
    //|  >> my_library::what_is_for_breakfast(...)               |//
    //|                                                          |//
    //| these comments look cool lol                             |//
    //|                                                          |//
    //| the following syntax with the curly braces is a nested   |//
    //| path, and allows for multiple imports from a common      |//
    //| scope without using up too much vertical space           |//
    //|                                                          |//
    //| the glob operator (*) allows us to import all entities   |//
    //| from a path into our current scope:                      |//
    //|                                                          |//
    //|  >> use std::collections::*;                             |//
    //|                                                          |//
    use library::my_library::{what_is_for_breakfast, Breakfast, Oatmeal};

    let breakfast = what_is_for_breakfast(
        Breakfast::oatmeal_with_apple_slices(Oatmeal::Regular)
    );

    println!("{}", breakfast);

    bar::proxy_hello("world");

    println!("2 + 2 = {}", bam::quux::sum(2, 2));
}

// 
// this re-exports the `FooBar` enum, so that anyone who brings this crate
// into scope can import `FooBar` into theirs without knowing its original
// path
// 
// doing this allows you to structure your code one way and present a different
// structure to its callers - this can become useful when considering the structure
// and logic of your domain and the contracts/functionality it provides, without
// needing to restructure or implement hacky solutions that could affect the design
// of your application logic
// 
// to quote the book:
// 
//> Re-exporting is useful when the internal structure of your code is different from
//> how programmers calling your code would think about the domain. (...) [Writing] our
//> code with one structure but [exposing] a different structure (...) makes our libary
//> well organized for programmers working on the library and programmers calling [it]. 
// 
pub use library::my_library::FooBar;
