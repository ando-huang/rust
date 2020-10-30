//this files goes over arrays
//arrays are fixed length, same data type
use std::mem;


pub fn run(){
    let mut nums:[i32; 5] = [1,2,3,4,5]; //must have the fixed size of elems, EXACT
    //let mut numbers: Vec<i32> = vec![1,2,3,4]; //now this is a vector definition

    //main difference is that we can edit the vector length
    //

    println!("{:?}", nums);

    //reassign a value, which is a valid op
    nums[2] = 23;

    println!("Get val at ind 2: {:?}", nums[2]);


    println!("Array uses {} bytes", mem::size_of_val(&nums));

    //get slice
    let slice: &[i32] = &nums[0..2]; //not including upper limit
    println!("Slice: {:?}", slice);

}