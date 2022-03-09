use std::env;
//This is an example for command line interface argument passing

pub fn run() {
    let args: Vec<String> = env::args().collect();     // using std::env; *imported above* helps shorten this line instead of having std::env::args....
    let command = args[0].clone();              // important to have the correct array index as this may cause a panick or may not but still run....
    let name = "User...";
    let status = "100%";

    //println!("Command: {:?}", command);

    if command == "hello" {
        println!("Hi {:?}, how are you?", name);
    } else if command == "status" {
        println!("Status is {:?}", status);
    }
}          

//when running from the command line, the argument is passed after the function name you are calling 
//ex. cargo run hello or cargo run status