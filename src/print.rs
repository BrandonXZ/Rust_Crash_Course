pub fn run(){
    //print to console
    println!("output from print.rs file...");

    //basic formatting
    print!("Number: {} ", 1);
    print!("{} is tesing placeholder function {}", "this ", "now...");

    //positional arguments
    println!("{} is from {} and likes to {}", "Joe", "Fl", "code");

    //Named Arguments
    println!("{name} likes to play {activity}", name = "joe", activity = "baseball");

    //placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    //Placeholder for debug trait -- also a tuple...
    println!("{:?}",(12, true, "hello"));

    //basic math 
    println!("10 + 10 = {}", 10 + 10);


}