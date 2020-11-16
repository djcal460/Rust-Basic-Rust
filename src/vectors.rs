//Vectors are resizable arrays

use std::mem;

pub fn run(){


        //array
        let mut numbers: Vec<i32> = vec![1,2,3,4];
        println!("{:?}",numbers);
    
        //reassign vals
        numbers[2] = 20;
        println!("{:?}",numbers);

        numbers.push(5);
        numbers.push(6);
        println!("Pushed: {:?}",numbers);


        numbers.pop();
        println!("Popped: {:?}",numbers);

    
        //get length
        println!("Vector length:{}",numbers.len());
    
        //get single val
        println!("Single val: {}",numbers[0]);
        
        //Vectors are heap allocated
        println!("Vector occupies {} bytes", mem::size_of_val(&numbers));
    
        //get slice
        let slice: &[i32] = &numbers[1..3];
        println!("Slice: {:?}",slice);

        //loop through vector values
        for x in numbers.iter(){
            println!("Number: {}", x);
        }

        //loop and mutate values
        for x in numbers.iter_mut(){

            *x *= 2;
        }
        println!("Numbers Vec: {:?}", numbers);
}