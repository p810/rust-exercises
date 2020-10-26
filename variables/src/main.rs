fn main() {
    let mut x = 5;
    output_value(x);
    x = 6;
    output_value(x);
}

fn output_value(x: u8) {
    println!("x = {}", x);
}
