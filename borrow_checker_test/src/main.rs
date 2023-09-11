fn main() {
    {
        let mut s = String::from("hello");
        println!("{s}");
        
        s.push_str(", world!");

        println!("{s}");
    }
    
    let s1 = String::from("hello 2");
    let mut s2 = s1;

    println!("{}", s2);

    borrowing_test(&s2);
    change_string(&mut s2);


    let phrase = String::from("hola pto");
    let word = first_word(&phrase);

    println!("{}", word);

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);

}


fn borrowing_test(s: &String){
    println!("se presto {s}");
}

fn change_string(s: &mut String){
    s.push_str(", world");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}



