//vectors -are resizeable arrays....

pub fn run(){
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];  // very similar to arrays only the list declaration uses <> instead of [] and the vec! keyword

    println!("{:?}",numbers);

    //Re-assign Value
    numbers[2] = 20;
    println!("{:?}", numbers);

    // add on to vector
    numbers.push(5);
    println!("{:?}", numbers);
    numbers.push(6);
    println!("{:?}", numbers);

    //pop off last value
    numbers.pop();
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

    println!("now for looping.....");

    //Loop through vector values
    for x in numbers.iter(){
        println!("Number: {}", x);
    }

    //Loop & mutate values 
    let mut numbers:[i32; 5] = [1, 2, 3, 4, 5];
    for x in numbers.iter_mut(){
        *x *= 2;
    }
    println!("Numbers Vec: {:?}", numbers);

}