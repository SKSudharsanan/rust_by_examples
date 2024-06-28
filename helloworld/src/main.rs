//this is starting point of any rust program
mod utils;
mod library;
mod print_utills;
fn main() {
    //we use println macro to print the hello world
    println!("Hello, world!");
    println!("I am a rustacean");
    utils::comments();
    library::create("sudharsanan".to_string());
    print_utills::formatted_print();
}
