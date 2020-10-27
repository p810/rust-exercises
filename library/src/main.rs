fn main() {
    use library::my_library::{what_is_for_breakfast, Breakfast, Oatmeal};

    let breakfast = what_is_for_breakfast(
        Breakfast::oatmeal_with_apple_slices(Oatmeal::Regular)
    );

    println!("{}", breakfast)
}