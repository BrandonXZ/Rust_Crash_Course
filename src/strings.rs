// Primitive str = Immutable fixed-length string somewhere in memory
// string = growable, heap-allocated data structure- modifiable**

pub fn run(){
    let mut hello = String::from("Hello ");
    println!("{}", hello);


    println!("{}", hello);

    // get length
    println!("Length: {}", hello.len());
    
    //push char
    hello.push('W');

    //push string
    hello.push_str("orld");
    hello.push('!');

    //Capacity in bytes
    println!("Capacity: {}", hello.capacity());
    //checks if empty
    println!("Is empty: {}", hello.is_empty());

    // contains or search
    println!("Contains 'world' {}", hello.contains("World"));

    //replace 

    println!("Replace: {}", hello.replace("World", "There")); // double quotes for stings more than 1 char in length

    //loop through string by whitespace
    for word in hello.split_whitespace(){
        println!("{}", word);
    }
    // Create string with capacity
    let mut s = String:: with_capacity(10);
    s.push('a'); // single quotes for single characters(char) only
    s.push('b');

    //Assertion testing ** nothing happens if it passes, if it fails then it will show an error thread panick
    assert_eq!(2, s.len()); // nothing
    assert_eq!(3, s.len()); // thread panicks when fails

    //this can also be used with capacity
    assert_eq!(10, s.capacity()); // nothing 
    assert_eq!(11, s.capacity()); // thread panick exits program also!!


    println!("{}", s);
}

//next concept  -> strings2.rs -> tuples.rs