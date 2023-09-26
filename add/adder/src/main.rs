use  add_one;
use rand;

fn main() {
    let num = 10;

    println!("num normally is {}, but sometimes is {}", num, add_one::add_one(num));
}
