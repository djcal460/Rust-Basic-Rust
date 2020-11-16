//Reference pointers point to a resource in memory

pub fn run(){

    //Primitive array
    let arr1 = [1,2,3];
    let arr2 = arr1;
    println!("Values {:?}",(arr1,arr2));

    /*
        With non primitives if you assign a variable to another variable,
        the first var will no longer hold that value. You need to use a reference (&)
        to point to that resource
    */

    //Vector array
    let vec1 = vec![1,2,3];
    let vec2 = &vec1;

    println!("Values: {:?}", (&vec1,vec2));

}