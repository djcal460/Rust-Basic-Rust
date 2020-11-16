//functions

pub fn run(){

    //pass in
    greeting("Hello", "Derek");
    
    //bind function values to variables
    let get_sum = add(5,5);
    println!("Sum: {}",get_sum);

    //Closure
    let n3: i32 = 10;
    let add_nums = | n1:i32, n2:i32 | n1 + n2 + n3;
    println!("C Sum: {}", add_nums(3,3));

}

pub fn greeting(greet: &str, name: &str){

    println!("{} {}, nice to meet you.", greet, name);
}

//return type a little different for rust functions
pub fn add (n1: i32, n2: i32) -> i32 {

    //in rust when you don't use semicolon, it means that's the intended return
    n1+n2

}