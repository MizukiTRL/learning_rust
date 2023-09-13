use std::io;

fn main() {

    //1:
    println!("1: printing primes in a range 1..=n");
    
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("input read");

    let last_num: i32 = input.trim().parse().expect("convert into int");

    print_primes(last_num.clone());

    //2:
    println!("2: printing a string but inversing each word");

    input.clear();

    io::stdin().read_line(&mut input).expect("input read");

    reverse_string_by_word(&mut input);

}

fn is_prime(n: i32)->bool{
    match n {
        1             => false,
        2 | 3 | 5 | 7 => true,
        _             => {
            for i in 2..=n/2{
                if n % i == 0{
                    return false;
                }
            }
            true
        }
    }
}

fn print_primes(n: i32){
    let mut prime_nums: Vec<i32> = Vec::new();

    for i in 1..=n{
        match is_prime(i) {
            true => prime_nums.push(i),
            false => (),
        }
    }

    for num in prime_nums{
        println!("{num}");
    }

}

fn reverse_string_by_word(s: &mut String){

    let mut space_i: Vec<usize> = Vec::new();

    for (i, c) in s.as_bytes().iter().enumerate(){
        match c {
            c if c.is_ascii_whitespace() => space_i.push(i),
            _    => (),
        }
    }

    let mut start: usize  = 0;
    let mut end: usize = 0;
    let mut reversed_string: String = String::new();

    for n in &space_i{
        end = n.clone();
        let reversed: String = s[start..end].chars().rev().collect();
        start = n.clone();
        reversed_string.push_str(&reversed.clone());
        reversed_string.push_str(" ");
    }
    if space_i.len()>0{
        let reversed: String = s[start..s.len()].chars().rev().collect();
        reversed_string.push_str(&reversed.clone());
        reversed_string.push_str(" ");
    }

    println!("{reversed_string}");

}