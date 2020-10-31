// 
// in the same way that `pub use` works, this can be used to re-export the contents
// of `./bam/quux.rs` to any calling scope
// 
// the call site would look like:
// 
// ```
// mod bam;
// 
// fn main() {
//   let sum = bam::quux::sum(2, 2);
//   println!("2 + 2 = {}", sum);
// }
// ```
// 
pub mod quux;