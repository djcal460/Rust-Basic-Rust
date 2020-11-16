pub fn run() {

    //print to console
    println!("Hello from print.rs file");

    //basic formatting
    println!("{} is from {}", "Brad","Mass");

    //positional arguments
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "code");

    //named arguements
    println!("{name} likes to play {activity}",name="John", activity="Baseball");

    //Placeholder Traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}",10,10,10);

    //Placeholder for Debug Trait Tuple
    println!("{:?}", (12,true,"Hello"));

    //Basic Math
    println!("10 + 10 = {}", 10+10);
}