//Conditionals - if/else etc... no ternary operators

pub fn run(){

    //easy stuff from tutorial, same as other langs
    let age = 18;
    let check_id: bool= false;
    let knows_person_of_age: bool = false;

    if age >= 21 && check_id || knows_person_of_age{
        println!("Bartender: What would you like to drink?");
    }else if age < 21 && check_id{
        println!("Bartender: Sorry you have to leave");
    }else{
        println!("Bartender: I'll need to see your id");
    }

    //shorthand
    let is_of_age = if age >= 21 {true} else {false};
    println!("is of age: {}",is_of_age );
}