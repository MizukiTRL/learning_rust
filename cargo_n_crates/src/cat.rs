pub struct Cat{
    name: String,
    age: u8,
}

impl Cat {
    pub fn new(name: String, age: u8) ->Self{Self{
        name: name,
        age: age,
    }}

    pub fn name()-> String => name;
}