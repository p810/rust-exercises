fn main() {
    println!("destructure_tuple(2, 2) = {}", destructure_tuple((2, 2)));

    countdown(5);

    println!("loop_over_numbers(5, 4, 3, 2, 1) = {}", loop_over_numbers([5, 4, 3, 2, 1]));

    println!("f_to_c(32) = {}", f_to_c(32));
    println!("f_to_c(68) = {}", f_to_c(68));
    println!("c_to_f(0)  = {}", c_to_f(0));
    println!("c_to_f(20) = {}", c_to_f(20));
}

fn destructure_tuple(input: (u8, u8,)) -> u8 {
    let (a, b) = input;

    a * b
}

fn countdown(mut from: u8) {
    while from > 0 {
        println!("{}!", from);

        from -= 1
    }

    println!("Lift off!");
}

fn loop_over_numbers(numbers: [u8; 5]) -> u8 {
    let mut result = 0;

    for number in numbers.iter() {
        result += number;
    }

    result
}

fn f_to_c(temp: i16) -> i16 {
    (temp - 32) * 5 / 9
}

fn c_to_f(temp: i16) -> i16 {
    (temp * 9 / 5) + 32
}