fn main() {
    println!("if_branch(4)              = {}", if_branch(4));
    println!("if_branch(12)             = {}", if_branch(12));
    println!("assign_with_if(5)         = {}", assign_with_if(5));
    println!("assign_with_if(12)        = {}", assign_with_if(12));
    println!("return_value_from_loop(8) = {}", return_value_from_loop(8));
}

fn if_branch(number: i8) -> i8 {
    return if number <= 9 {
        number - 1
    } else {
        number + 2 
    }
}

fn assign_with_if(number: i8) -> i8 {
    // the types must be the same in either arm
    // of the branch, otherwise there will be
    // an error
    let x = if number > 9 { 1 } else { 0 };
    x
}

fn return_value_from_loop(iterations: i8) -> i8 {
    let mut number = 0;

    // a value can be returned from a loop by
    // including an expression after the break
    // keyword
    let result = loop {
        if number == iterations - 1 {
            break number;
        }

        number += 1;
    };

    result
}
