fn main() {
    let my_ip = Ip::V4(String::from("127.0.0.1"));

    println!("My IP address is: {:?}", my_ip);

    let x = Message::Move {
        x: 128,
        y: 80,
    };

    // option a:
    if let Message::Move { y, x: z } = x {
        println!("x = {}, y = {}", z, y);
    }

    // option b (see method implementation for details):
    x.print_coordinates();

    let y = value_in_cents(Coin::Dime);
    println!("I currently have {} cents", y);

    let z = get_quote_from_quarter(Coin::Quarter(UsState::Alaska));
    println!("{}", z);

    let a = get_quote_from_quarter_using_if_let_shorthand(Coin::Quarter(UsState::Alabama));
    println!("{}", a);

    if let Some(number) = plus_one(Some(5)) {
        println!("plus_one(Some(5)) = {}", number);
    }
}

//
// enum Ip {
//     V4,
//     V6,
// }
// 
// struct IpAddress {
//     version: Ip,
//     address: String,
// }
//
// The above can also be modeled with the following, which says that
// each variant in the enum will have an associated String. This is
// more or less the behavior we want through the combination of our
// struct and enum. It also has the added benefit of allowing us to
// store the variant's values as separate types - for example, Ip::V4
// could be four u8's while Ip::V6 was a string. This is not possible
// using a struct because it would need a specific type, alongside the
// identifying variant of Ip. Rust's standard library stores each one
// as a struct of its own, i.e. IpV4Addr and IpV6Addr.
//
#[derive(Debug)]
enum Ip {
    V4(String),
    V6(String),
}

//
// The following enum has an anonymous struct inside it, as well as
// an empty variant and a couple which store types. Anonymous structs
// evidently can't be referenced like structs usually can, with dot
// notation - instead, a match or if let statement must be used to
// destructure the value from the variant. See above in main().
//
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32, },
    Write(String),
    ChangeColor(i32, i32, i32,)
}

// 
// Methods can also be implemented for enums, using the handy impl
// construct:
// 
impl Message {
    fn print_coordinates(&self) {
        match self {
            Message::Move { x, y } => println!("x = {}, y = {}", x, y),
            _ => println!("Message does not have coordinates"),
        };
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

enum UsState {
    Alabama,
    Alaska,
    // ...
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(_) => 25,
    }
}

fn get_quote_from_quarter(coin: Coin) -> &'static str {
    match coin {
        // the UsState variant belonging to our Coin::Quarter can be
        // destructured in the arm of the match, to be used in its
        // expression - and like above, it can be an underscore to
        // simply ignore the value altogether
        Coin::Quarter(state) => {
            match state {
                UsState::Alabama => "In God we trust",
                UsState::Alaska  => "It's cold up here",
            }
        }
        _ => "The given coin doesn't have a quote because it isn't a quarter",
    }
}

//
// This could also be done with if let (...) { ... } else { ... }
// but I'm choosing to omit it here out of habit since returning
// early allows us to skip the else
// 
fn get_quote_from_quarter_using_if_let_shorthand(coin: Coin) -> &'static str {
    if let Coin::Quarter(state) = coin {
        return match state {
            UsState::Alabama => "In God we trust",
            UsState::Alaska  => "It's cold up here",
        }
    }

    "The given coin doesn't have a quote because it isn't a quarter"
}

fn plus_one(x: Option<isize>) -> Option<isize> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}