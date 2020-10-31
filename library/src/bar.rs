// 
// using a semicolon after the `mod` keyword (as opposed to opening a block) will cause
// the contents of the module in `./bar/foo.rs` to be loaded into the scope of this file
// 
// similarly, we can go further down into a nested directory with the same syntax as if it
// were defined in a nested module in this file, e.g. `mod foo::bam::quux` -> `./bar/foo/bam/quux.rs`
// 
// `pub mod` can be used to re-export the given module in the same way that `pub use` works
// 
mod foo;

pub fn proxy_hello(name: &str) -> () {
    foo::greeter::say_hello_to(name);
}