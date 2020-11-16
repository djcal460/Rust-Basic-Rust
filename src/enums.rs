//enums are types which have  few definite values

//create movement enum
enum Movement{
    //Variants
    Up,
    Down,
    Left,
    Right
}

fn move_avatar(m: Movement){

    //Perform action depending on info

    //use match which is similar to swtich in rust
    match m{
        Movement::Up => println!("Avatar moving up"),
        Movement::Down => println!("Avatar moving down"),
        Movement::Left => println!("Avatar moving left"),
        Movement::Right => println!("Avatar moving right")
    }
}

pub fn run(){

    //set some varaibles
    let avatar1 = Movement::Left;
    let avatar2 = Movement::Up;
    let avatar3 = Movement::Right;
    let avatar4 = Movement::Down;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
    
}