#![allow(unused)]

mod tests;
struct Cacher<T> 
where 
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

#[derive(Debug,PartialEq, Eq)]
struct Shoe{
    size: u32,
    style: String,
}

fn shoes_that_fit(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe>{
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

struct Counter{
    count: u32,
}

impl Counter {
    fn new() -> Counter{
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5{
            self.count += 1;
            Some(self.count)
        }else {
            None
        }
    }
}

fn main() {
    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));

    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for i in v1_iter{
        println!("{i}");
    }

    let sum: i32 = v1.iter().sum();

    println!("{sum}");

    let idk = v1.iter().rev();

    let shoes = vec![Shoe{size: 10, style: format!("shoe")}, Shoe{size: 11, style: format!("shoe")},Shoe{size: 10, style: format!("shoe")}];

}
