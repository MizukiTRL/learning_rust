fn main() {
    let str1 = "asbcs";
    let str2 = "abc";

    let long = longest(str1, str2);

    println!("longer: {}", long);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str{
    if x.len() > y.len(){
        x
    }else{
        y
    }
}
