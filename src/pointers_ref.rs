//reference pointer - point to a resource in memory

pub fn run(){
                            //Primitive Array
    let arr1 = [1,2,3];
    let arr2 = arr1;

    println!("Values: {:?}", (arr1, arr2));

    // With non-primitives, if you assign another variable to a piece of data, the first variable will no longer hold that value.
    //you'll nee to use a reference (&) to point to the resource

    println!("....New section.....Vectors");
    //Vectors
    let vec1 = vec![1,2,3];
    let vec2 = &vec1;                    // note the & reference operator
    println!("Values: {:?}", (&vec1, vec2)); // also here!!

}