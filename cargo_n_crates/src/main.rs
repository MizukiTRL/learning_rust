mod cat;

use cat::Cat;
fn main() {
    println!("Hello, world!");

    let campanita = Cat::new("campana", 15);

    println!("{}", campanita.name)
}
