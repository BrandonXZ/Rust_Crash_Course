// Conditionals -Used to check the condision of something and act //basic if statement syntax use 

pub fn run(){
    let mut age:u8 = 18;
    let check_id: bool = true;
    let knows_person_of_age = true;

      // And operator is the same as other languages
    if age >= 21 && check_id {
        println!("Bartender says yes....\n");
    } else if age < 21 && check_id{
        println!("Bartender says NO!!!!!! \n");
    } else {
        println!("ID please: \n");
    }

    //  Or operator is the same as other languages
    age = 21;
    println!("...New Section...\n");
    if age >= 21 && check_id || knows_person_of_age {
        println!("Bartender says yes....\n");
    } else if age < 21 && check_id{
        println!("Bartender says NO!!!!!!\n");
    } else {
        println!("ID please: \n");
    }

    // note: there are no ternary operator in rust but short hand is possible for If statments 
    println!("...New Section...\n");
    let is_of_age = if age >= 21 {true} else {false};
    println!("Is of age: {}",is_of_age)
}

//next concept -> loops.rs
