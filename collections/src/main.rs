mod vector;

fn main() {
    let vec = vector::vectors_are_instantiated_with_associated_function_new_or_vec_macro();

    println!("1 + 2 = {}", vector::reading_vectors_is_done_with_indexing_syntax_or_get(&vec));

    let vec = vector::looping_over_vectors_with_for(vec);

    println!("2 + 3 = {}", vector::reading_vectors_is_done_with_indexing_syntax_or_get(&vec));
}
