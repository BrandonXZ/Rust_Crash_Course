//arrays - fixed lists where elements are the dame data types

pub fn run(){
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];  // will not run with wrong number of list elements or a wrong element type like a string

    println!("{:?}",numbers);

    //Re-assign Value
    numbers[2] = 20;
    println!("{:?}", numbers);

    // Get single val
    println!("Single Value: {}", numbers[0]);

    // Get array length
    println!("Array length: {}", numbers.len());

    // arrays are stack allocated 
    println!("Arrays are stack allocated...");
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));

    let numbers: [i32; 4] = [1, 2, 3, 4];
    println!("Arrays are stack allocated...");
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));

    //get slice 
    let slice: &[i32] = &numbers[0..2]; // the slicing magic goes here....[ 0..2]
    println!("Slice: {:?}", slice);
    let slice: &[i32] = &numbers[1..3]; // magic....
    println!("Slice: {:?}", slice);



}