// touples group together values of different types 
// 12 elements max
pub fn run(){
    let person:(&str, &str, i8) = ("Joe", "FL", 21);
    println!("{} is from {} and is {}", person.0, person.1, person.2);


}