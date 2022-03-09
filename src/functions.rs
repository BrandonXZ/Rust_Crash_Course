//Functions - used to store blocks of code for reuse, also called "methods" in java like languages 

pub fn run(){
    greeting("hello", "person");

    //Binding function values to variables
    let get_sum = add(5, 5);
    println!("Sum: {}", get_sum);

    //Closure  variables outside are accessible!!
    let n3 = 10;
    let add_nums = |n1:i32, n2:i32| n1 + n2 + n3;
    println!("C sum: {}", add_nums(3,3));
}

fn greeting(greet: &str, name: &str){
    println!("{} {}, nice to meet you!!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32{
    n1 + n2

}

//next concept -> conditionals.rs