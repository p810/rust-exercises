pub mod greeter {
    pub fn say_hello_to(name: &str) -> () {
        println!("{}", format!("Hello, {}!", name));
    }
}
