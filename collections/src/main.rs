mod vector;
mod string;

fn main() {
    vector_examples();

    string_examples();
}

fn vector_examples() {
    let vec = vector::vectors_are_instantiated_with_associated_function_new_or_vec_macro();

    println!("1 + 2 = {}", vector::reading_vectors_is_done_with_indexing_syntax_or_get(&vec));

    let vec = vector::looping_over_vectors_with_for(vec);

    println!("2 + 3 = {}", vector::reading_vectors_is_done_with_indexing_syntax_or_get(&vec));

    let mut x = vec![2];

    vector::get_mut_returns_a_mutable_reference_for_an_element(&mut x);

    println!("2 + 10 = {}", x[0]);

    vector::slicing_can_be_used_to_get_a_read_only_array(&x);

    let y = vector::with_capacity_can_be_used_to_specify_vector_capacity();

    println!("Length of y = {}", y.len());
}

fn string_examples() {
    let mut x = string::string_type_shares_methods_with_other_collection_types();
    x.push_str("Hello, world!");

    let y = string::types_implementing_display_can_be_represented_as_strings();

    assert_eq!(x, y);

    let mut z = String::new();
    z = string::strings_can_grow_and_change(&mut z);
    println!("{}", z);

    let a = String::from("Test");
    println!("a = {}", string::concatenation_with_plus_operator_returns_new_string(a));

    let b = String::from("Hello world!");
    println!("{:?}", string::get_reversed_chars_from_string(&b));
    println!("{:?}", string::get_bytes_from_string(&b));
}