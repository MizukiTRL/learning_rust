#![allow(unused)]

use std::collections::HashMap;
fn main() {
    let a = [1, 2, 3];
    let mut b: Vec<i32> = Vec::new();

    b.push(1);
    b.push(4);
    b.push(3);

    let mut b2 = vec![1, 2, 3];

    match b.get(5) {
        Some(n) => println!("{n} :3"),
        None          => println!("index out of bounds")
    }

    let mut v: Vec<Sheet> = Vec::new();

    v.push(Sheet::Number(1));
    v.push(Sheet::Text(String::from("hola")));

    for i in v{
        match i {
            Sheet::Number(n) => println!("number {n}"),
            Sheet::Text(t) => println!("text {t}")
        }
    }

    let s1 = String::new();
    let s2 = "string";
    let s3 = s2.clone().to_string();
    let s4 = String::from("String");

    let mut s = String::from("foo");

    s.push_str("bar");
    s.push('!');

    let blue = String::from("blue");
    let yellow = String::from("yellow");

    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(blue, 10);
    scores.insert(yellow, 20);

    let team_name = String::from("blue");
    let score = scores.get(&team_name);

    for (key, value) in scores{
        println!("key: {key}\nvalue: {value}");
    }

}

enum Sheet{
    Number(i32),
    Text(String),
}
