fn main() {
    let s1 = String::from("Hello, world!");
    println!(
        "s1 is moved here and goes out of scope unless returned by the function and reassigned: {}",
        calculate_length_by_moving_heap_data(s1)
    );


    let s2 = String::from("Hello, universe!");
    println!(
        "s2 is referenced by the function and will automatically be returned to the scope of main() when it leaves the other: {}",
        calculate_length_without_moving_heap_data(&s2)
    );

    println!("We can still use s2 without it throwing an error: {}", s2);

    let mut subject = String::from("Mom");
    println!("subject = {}", subject);
    println!("We can modify the subject variable into a greeting: {}", say_hello_to(&mut subject));

    use_mutable_and_immutable_references();

    let sentence = String::from("Hello, mom and dad!");
    println!("Index of \"Hello\" in our sentence: {}", index_of_first_word(&sentence));
    // I'm a bit confused about this part... the docs says it's better to use slicing here
    // because it means we don't have to maintain a bunch of variables, like the starting and
    // ending index, which could be invalidated by changes to the borrowed variable. so I guess
    // those variables would be useless due to the fact that the string changed. so iiuc the 
    // solution here then is to slice the string early, and have the function return the substring
    // instead of relying on the indices. this is better for cases like mutable borrows
    println!("Slicing guarantees that the reference remains valid in lieu of mutations: {}", first_word_by_slicing(&sentence));
}

fn index_of_first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    // s.as_bytes()     -> [u8]         - An array of bytes representing the chars in the string
    // bytes.iter()     -> Iter<'_, u8> - An iterator that allows us to traverse the bytes
    // iter.enumerate() -> (u8, string) - Another iterator that provides tuples which contain the
    //                                    index and a reference for the value
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// 
// it's preferable to use str over String here because it makes the
// parameter more general and allows us to use more functionality -
// a String will be recognized as a str, but not the other way around.
// also, the reason we can use &str here is because there's an argument
// that the size can be derided from - otherwise we'd need to specify
// a lifetime thingy for the string.
// 
fn first_word_by_slicing(s: &str) -> &str {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

// according to the docs, a scenario like this would cause a dangling pointer
// in other languages because the first function would return a pointer to an
// address space that would have already been deallocated and dropped, thus,
// some kind of error would occur with the program
//
// a lifetime can be applied to the returned reference, but I haven't gotten to
// that point yet, so I can't cover it here
//
// fn rust_prevents_dangling() -> &String {
//     let s = String::from("");
//     &s
// }
//
// fn would_cause_a_dangling_pointer() {
//     let reference_that_points_to_deallocated_address = rust_prevents_dangling();
// }

// The following function will move the heap data that belongs to s1
// into s2, which invalidates s1 and causes Rust to throw an error if
// we try to use it; data that has a fixed length will not have this
// problem because it is copied and a new value is pushed onto the stack.
// Simple scalar values can implement Copy and be copied, but any type
// whose data must be allocated to the heap will need to implement clone()
// in order for its data to be cloned to a new address space. 
//
// fn moving_variable_on_heap_invalidates_first_variable() -> () {
//     let s1 = String::from("Hello, world!");
//     let s2 = s1;
//     println!("{}", s1);
// }

fn calculate_length_without_moving_heap_data(s: &String) -> usize {
    s.len()
}

fn calculate_length_by_moving_heap_data(s: String) -> usize {
    s.len()
}

fn say_hello_to(subject: &mut String) -> &String {
    subject.push_str(", hey what's up?");

    subject
}

fn use_mutable_and_immutable_references() {
    let mut s = String::from("Hello, mom and dad!");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);

    // since r1 and r2 are last used above, apparently the scopes for those
    // two and this mutable ref don't overlap, so we can do this - but if r1 or
    // r2 were used again after assigning this mutable reference then Rust would 
    // panic, because there could be no guarantee of its immutability. (I thought
    // that they'd be in scope until the function ended, but I guess the docs are
    // talking about s/t else?
    // https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#mutable-references
    // nvm - to quote the docs: "(...) a reference's scope starts from where it is introduced
    // and continues through the last time that reference is used." This is different from var
    // scope I believe
    let r3 = &mut s;
    println!("Again but mutable this time: {}", r3);
}