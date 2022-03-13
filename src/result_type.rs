// An explanation of the result type in custom fn's so youre not left using only generics of match
// usually used in cases with more than 1 error type possibility 

pub fn run(){

    let my_num = 50;
    fn is_it_fifty(num:u32) -> Result<u32, &'static str> {
        let error = "It didn't work";
        if num ==50 {
            Ok(num)
        } else {
            Err(error)
        }


    } match is_it_fifty(my_num) {
        Ok(_v) => println!("Good! my_num is 50"),
        Err(_e) => println!("Error. my_num is {:?}", my_num)
    }
}
     //final note:for compiler error "expected this, found enum,etc" quickest/easiest thing to try is 
     //try adding .expect or .unwrap to see if remedy is found 
//next concept -> user_defined.rs