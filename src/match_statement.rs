//These are practice modules for rust error handling 
//Using generic result types, box dyn, user defined errors, crate "inherited" errors


use std::io::{stdin, self, Write};



pub fn run() {
    the_jist();
    match_jist()
}

                          //the jist will allow you to enter anything and convert to string 
                          //match jist only wants a number<f64> and maps incorrect input to error msg


fn the_jist(){
    let mut user_input = String::new();
    print!("1. Enter a number: ");


                       //**cases when you want to use .expect or .unwrap**
    //"flush" - standard output from printed line //almost guaranteed not to crash from std output
    //"flush" - is used when you are really not expecting any errors and not dealing with it 
    //"expect" - is similar, when you are pretty sure its going to work
    //"readline" - is fine when you are expecting a string, 
    //When pretty sure it will work its followed with expect


    io::stdout().flush().unwrap(); //double check to see if this is converting to string

    stdin().read_line(&mut user_input)
    .expect("Did not enter a correct string");

            //"unwrap" isn't the best to use because it  will panic crash when unknown type is returned 
           //"unwrap" is mainly used for testing because its easy to use.

//    let my_number: f64 = user_input.trim().parse().unwrap(); 


    let my_number: String = user_input.trim().parse().expect("You probably didn't enter a number...");
    //"trim" removes extra line before/after input,text.etc
    //"expect" is a little better because you can enter an error message... but still crashes.
    //Error needs to be handled in order not to crash  -> see match jist
    println!("You entered a number, it was: {:?}", my_number);

}

fn match_jist(){
    let mut my_string = String::new();
    println!("2. Enter a number: ");
    io::stdout().flush().unwrap();


                                            //whatever value loop returns is stored in my_num
    let my_num = loop { 
        my_string.clear();                //clear to ensure string is empty then read user input into it
                                         //"match" keyword, trim to remove extra ln, then parse/convert
                                         //to 64-bit, parse::<64> format due to "match",can be .parse()
         stdin().read_line(&mut my_string)
        .expect("Did not enter a correct string");
                                                //w.e is returned from this, match takes result
        match my_string.trim().parse::<f64>(){ // and allows handling of what is returned 
            Ok(_s) => break _s,                //anything after break is what gets returned "_s" is var name
            Err(_err) => println!("Try again. Enter a number.")
        }
    };
    println!("You entered {:?}", my_num);
    
}

// next concept -> result_type.rs