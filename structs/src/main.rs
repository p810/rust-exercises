fn main() {
    let x = get_user("anna", "anna@gmail.com", false, true);
    println!("Hello, {}! Your email is {}.", x.username, x.email);
    let y = get_user_by_update_syntax(x);
    println!("We got an updated instance by unpacking the initial one into a partial instance: {}", y.email);

    let p1 = get_point(1, 2, 3);
    println!("Your coordinates are: {},{},{}", p1.0, p1.1, p1.2);

    let z = Rectangle {
        width: 32,
        height: 32,
    };
    println!("My rectangle: {:?}", z);
    println!("The area of my rectangle is: {}", area(&z));
    println!("The area of my rectangle using a method: {}", z.area());
}

//
// the struct uses String for its string properties because we want it
// to own the values, as borrowing the values from another source could
// cause the data to change or become invalidated at some point in its
// lifetime. we could use &str, but it would need a lifetime specifier
// like 'static, which is in a later chapter I haven't gotten to yet.
// according to the docs, lifetimes guarantee that data referenced by
// the structure remains valid for as long as the struct is. so I guess
// that a lifetime is probably a way for types to keep data alive to
// prevent invalidation?
//
struct User {
    username: String,
    email: String,
    signed_in: bool,
    is_active: bool,
}

fn get_user(username: &str, email: &str, signed_in: bool, is_active: bool) -> User {
    User {
        username: String::from(username),
        email: String::from(email),
        signed_in, is_active,
    }
}

fn get_user_by_update_syntax(other_instance: User) -> User {
    User {
        username: String::from("p810"),
        email: String::from("im@9933.at"),
        ..other_instance
    }
}

#[derive(Debug)] // implementing this annotation allows the struct to be printed
                 // with println!() and {:?} type interpolation - regular curly
                 // braces would require implementing Display or deriving s/t else
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // this is an associated function, the likes of which are most commonly used to implement
    // constructors; they don't do work on a self instance and are accessed with PHP's static
    // method call operator (e.g. Rectangle::square(...))
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    // &self is used because we want to borrow the instance without 
    // changing anything about it; &mut self would be for mutations,
    // and self would be used if we wanted to prevent the caller from
    // reusing the instance again, typically done when transforming an
    // instance into something else (https://doc.rust-lang.org/book/ch05-03-method-syntax.html)
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn area(of: &Rectangle) -> u32 {
    of.height * of.width
}

//
// the following tuple struct lacks names for its properties
// but represents a geocoordinate point, and is equivalent to
// the following:
//
// struct Point {
//     x: i32,
//     y: i32,
//     z: i32,
// }
// 
// (it's probably better to explicitly name those for readability
// though, since coordinate.0 is not very intuitive unless you
// destructure it into named variables at the call site)
//
struct Point(i32, i32, i32);

fn get_point(x: i32, y: i32, z: i32) -> Point {
    Point(x, y, z)
}