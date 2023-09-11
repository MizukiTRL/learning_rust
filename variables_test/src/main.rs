fn main() {
    let  mut x = 5;
    println!("el numero {x} es inmutable");
    x=6;
    println!("no sirve {x}");

    const THREE_HOURS_IN_SECONDS: i32 = 3 * 60 * 60;

    println!("in three hours there are {THREE_HOURS_IN_SECONDS} seconds");

    let y = 6;

    println!("the value of y is {y}");

    //shadowing

    let y = y + 1;

    println!("the value of y is {y}");

    {
        //scope
        let y = y * 2;
        println!("the value of y in the scope is {y}");
    }

    println!("the value of y is {y}");

    //tuples

    let tup = (500, 6.4, 1);
    //destructing

    let (_x, y, _z) = tup;

    println!("The value of y in a tuple is: {y}");

    //arrays

    let a = [1, 2, 3, 4, 5];

    let b: [i32; 5] = [1, 2, 3, 4, 5];

    another_func(7);

    println!("The value of number is: {}", conditional_on_let());

    counter();

    while_array_traversal(a);

    for_array_traversal(b);

}

fn another_func(x: i32){
    println!("this is another function that prints the num: {x}")
}

fn conditional_on_let()->i32{
    let condition = true;
    let number = if condition { 5 } else { 6 };

    return number;
}

fn counter(){
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn while_array_traversal(array: [i32; 5]){
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", array[index]);

        index += 1;
    }
}

fn for_array_traversal(array: [i32; 5]){
    for n in array{
        println!("the value in for loop is: {n}");
    }

    for n in (0..array.len()).rev(){
        println!("the value in for loop reversed is: {n}");
    }
}
