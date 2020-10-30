//this files goes over arrays
//arrays are fixed length, same data type
use std::mem;


pub fn run(){
    
    let mut nums: Vec<i32> = vec![1,2,3,4]; //now this is a vector definition

    //main difference is that we can edit the vector length
    //

    println!("{:?}", nums);

    //reassign a value, which is a valid op
    nums[2] = 23;

    nums.push(22);
    nums.push(10);//adds on to back

    nums.pop(); //removes last value

    println!("Get vals: {:?}", nums);


    println!("Array uses {} bytes", mem::size_of_val(&nums));

    //get slice
    let slice: &[i32] = &nums[0..2]; //not including upper limit
    println!("Slice: {:?}", slice);

    for x in nums.iter() {
        println!("Number: {}", x);
    }

    for x in nums.iter_mut() {
        *x *=2; //must use the pointer so we actually edit the value outside this loop
    }
    println!("Number: {:?}", nums);

}