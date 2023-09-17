#![allow(unused)]

struct Point<T, U>{
    x: T,
    y: U,
}

fn main() {
    let num_list = vec![1, 3 ,7, 3, 5];
    let num_list2 = vec![21535, 5436 ,4325, 68582336, 645];
    let char_list = vec!['a', 'f', ',', 'd'];

    println!("el mayor es {}", get_largest(&num_list));
    println!("el mayor es {}", get_largest(&num_list2));
    
    println!("el mayor es {}", get_largest(&char_list));

    let p1 = Point{x: 2, y: 4.6};
}

fn get_largest<T: PartialOrd + Copy>(vec: &Vec<T>) -> T{
    let mut mayor = vec[0];

    for n in vec{
        if mayor < *n{
            mayor = *n;
        }
    }

    mayor
}
