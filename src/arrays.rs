//Arrays -- fixed list where elements are the same data types

use std::mem;

pub fn run(){

    //array
    let mut numbers: [i32;5] = [1,2,3,4,5];
    println!("{:?}",numbers);

    //reassign vals
    numbers[2] = 20;
    println!("{:?}",numbers);

    //get length
    println!("Array length:{}",numbers.len());

    //get single val
    println!("Single val: {}",numbers[0]);
    
    //Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    //get slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}",slice);

}