//Variables hold primitive data or refereces to data
//Variables are immutable by default
//Rust is a block-scoped language

pub fn run(){

    //vars mutable 
    let name = "Derek";
    let mut age = 37;
    println!("My name is {} and I am {}",name, age);
    age = 38;
    println!("My name is {} and I am {}",name, age);

    //Define constant (must add a type)
    const ID: i32 = 001;
    println!("ID: {}",ID);

    //multiple vars at once
    let ( my_name, my_age) = ("Derek", 37);
    println!("{} is {}", my_name, my_age);



}