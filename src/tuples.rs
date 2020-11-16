//tuples group together values of different types
// max 12 elements

pub fn run(){

    //create and print tuple
    let person: (&str,&str,i8)= ("Derek", "Mich", 37);
    println!("{} is from {} and is {}",person.0, person.1, person.2 );
    
}