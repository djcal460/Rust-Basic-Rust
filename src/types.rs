/*
    **Primitive Types**
    Integers: u8,i8,u16,i16,u32,i32,u64,i64,u128,i128 (num of bit in memory)
    Floats: f32, f64
    Boolean: (bool)
    Characters: (char)
    Tuples / Arrays / Vectors
*/ 

//Rust is statically typed langauge means must know the types of all vars at compiles time.
//However, the compilier can usually infer what type we want to use based on the value and how it's used


pub fn run(){

    //default is i32
    let x = 1;

    //default is f64
    let y = 2.5;

    //add explicit type
    let z: i64 = 23542345345353;

    //Find max size
    println!("Max i32: {}",std::i32::MAX);
    println!("Max i32: {}",std::i64::MAX);

    //boolean
    let is_active = true;
    let is_greater: bool = 10 < 5;
    let a1 = 'a';
    let face = '\u{1F600}';
    println!("{:?}",(x,y,z,is_active,is_greater,a1,face));

}