mod vector;

fn main() {
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
