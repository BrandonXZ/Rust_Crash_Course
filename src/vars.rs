// Variables hold primitive data or referenced to data 
// variables are immutable by default
// rust is a block-scoped language 

pub fn run(){
    let name = "Brandon";
    let mut age = 21; // keyword mut allows variable to be changed 
    println!("my name is {} and I am {}", name, age);
    age = 25;

    println!("my name is {} and I am {}", name, age);

    //Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID );

    //assign multiple vars
    let ( my_name, my_age ) = ("Brandon", 21);
    println!("{} is {}", my_name, my_age);
    
}

//next concept -> types.rs