//pointers and ref

pub fn run(){
    // lets learn about pointers!!
    //given a primitive value, we can have a pointer to it. 
    //let array1 = [1,2,3,4];
    //let array2 = array1;

    let vec1 = vec![1,2,3,4];
    let vec2 = &vec1;


    println!("Values: {:?}", (&vec1, vec2));
}

