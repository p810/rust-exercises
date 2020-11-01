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
- Strings can be thought of in three ways, using "test" as an example:
    1. As a collection of **bytes**, i.e. the vector that stores the character's bytes - `[116, 101, 115, 116]`
    2. As a collection of **characters** (`char`s) - `['t', 'e', 's', 't']`
    3. As a **grapheme cluster**, which is what we can think of as the letters making up a word - if any of the characters in "test" stored more than one byte, then the collection of characters above would be different, but the grapheme cluster would remain the same
- String slices can be done using indexing syntax (e.g. `&var[0..1]`) but it can cause an exception to be thrown if the index doesn't cover the entirety of a character 
    * For example `д` being represented by two byes means that `&var[0..1]` would return that character if it were the first one in the string, but if it were the second, and preceded by a single byte character, then Rust would panic because it would have an incomplete character; the error thrown is something like, "byte index *x* is not a char boundary; it is inside *y*"

### Hash maps
- Hash maps are considered **homogenous** (like vectors are) because they must contain keys and values of the same type; in this way, they are different from something like an object in JavaScript
