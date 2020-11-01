## Notes

### Vectors
- The **capacity** of a vector is the amount of memory allocated for any future elements that will be added onto it
- A vector's **length** refers to the number of elements currently on it
- When the length of a vector surpasses its capacity, the vector grows, and its elements are reallocated to a new address in memory
- Vectors are considered **contiguous** because all of the elements contained within are beside each other in memory

### Strings
- Strings are a collection type because it's a collection of bytes, with methods for working with them as text (what I typically think of strings as)
- String slices (`str` and `&str`) are read only references to UTF-8 string data, and belong to the core language
- The `String` type is provided by the standard library and represents a growable, mutable, UTF-8 string
- There are other string types as well, since `String` is implemented as a type and can be extended
- The `String` type is a wrapper over a `Vec<u8>`
- Because strings in Rust are UTF-8, an index might not always correlate to a proper Unicode value, because they can contain more than one byte per character
    * Example: `д` is represented by two bytes, and as the first character in a string named `x`, one might think that `x[0]` would equal `д`, but it would only be that character's first byte which is 208
    * Since this can cause unexpected results when using index access, Rust decided not to allow indexing on strings 
