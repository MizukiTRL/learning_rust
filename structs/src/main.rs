struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    println!("Hello, world!");
    
    let campanita = Cat{
        name : String::from("campanita"),
        breed : String::from("mixed"),
        age : 15,
    };

    let mia = Cat{
        name: String::from("mia"),
        ..campanita
    };

    println!("{} {}", mia.name, mia.age);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let rectangle = (500, 1000);

    println!("gato: {:?}", mia);

    mia.printData();
}
#[derive(Debug)]
struct Cat{
    name: String,
    breed: String,
    age: i8,
}

impl Cat{

    fn printData (&self){
        println!("Name: {}\nBreed: {}\nAge: {}", self.name, self.breed, self.age);
    }

    fn new_cat(name: String, breed: String, age: i8) -> Self{Self {
        name: name,
        breed: breed,
        age: age 
    }}
}
