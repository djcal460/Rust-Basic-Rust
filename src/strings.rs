//Primitive str = Immutable fixed-length string somewhere in memory
//String = Growable, heap-allocated data structure - Use when you need to modify string data

pub fn run(){

    //dynamic string
    let mut hello = String::from("Hello ");
    println!("{}", hello);

    //get length
    println!("Length: {}", hello.len());

    //push a char onto dynamic string
    hello.push('W');
    //push str onto dynamic string
    hello.push_str("orld");
    
    //capacity in bytes
    println!("Capacity: {}",hello.capacity());
    
    //check if empty
    println!("Is Empty: {}",hello.is_empty());

    //contains
    println!("Contains: {}",hello.contains("World"));

    //replace
    println!("Replace: {}",hello.replace("World","There"));
    
    //loop through string
    for word in hello.split_whitespace(){
        
        println!("{}",word);
    }

    println!("{}", hello);

    //create string with capacityu
   let mut s = String::with_capacity(10);
   s.push('a');
   s.push('b');
   println!("{}", s);

   //assertion testing
   assert_eq!(2,s.len());
   assert_eq!(10,s.capacity());


}