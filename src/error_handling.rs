// This module goes a little deeper than the  first 14 modules
#[allow(dead_code)]
//Errors are split into 2 categories, recoverables and non-recoverables...will the program crash after error or is there a way to salvage and try again?
//Rust doesn't use the idea of null, instead it used the option and result enums
//unrecoverable examples are program stop or thread panick

pub fn run(){
    // let v = vec![1,2];                //Remove comment char to see example of a thread panick. index out of bound, there are only 2 options but we're calling the 50th option
    // v[50]                            //Backtrace=1 will walk you through how this happened(for more complex functions)
    exit(1);
    exit(0); 
    exit2(Some(1));
    exit2(Some(10));
    exit2(None);
    exit2(Some(0));
} 

fn exit(x: i32){
    if x == 0 {
        panic!("we got a 0");              //When running, this will panic and exit saying "we got a 0"
    }
    println!("things are fine");           //This code will run fine
}
                  

                                        //Most error types are not serious enough to close the program entirely. In most cases when we want to try again or 

                                        //Seek user intervention, we use the result type mentioned in the intro
#[allow(dead_code)]
enum Result<T,E>{
    Ok(T),
    Err(E)
}
#[allow(dead_code)]
enum Options<T>{
    Some(T),
    None, 
}

fn exit2(x: Option<i32>){
    match x{
        Some(0) => panic!("we got a 0!!"),
        Some(x)=> println!("we got a {} things are fine!", x),
        None => println!("we got nothing "),
    }

}