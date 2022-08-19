fn main() {
    use greet::*;
    hello();
    goodbye();
}

mod greet {
    pub fn hello() {
        println!("Hello");
    }

    pub fn goodbye() {
        println!("Goodbye");
    }
}
