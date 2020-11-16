//

use std::env;

pub fn run(){

    //use std::env to collect any cli args
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "Derek";
    let status = "100%";

    //vector 
    println!("Args: {:?}",args); //always starts with "target/debug/package_name"

    println!("Command: {}",command);  

    if command == "Hello"{
        println!("Hi {}, how are you?", name);
    }else if command == "status"{
        println!("Status is {}",status);
    }
}