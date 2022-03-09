//Structs - used to create custom data types 



                                                         //Traditional Struct
//struct Color{
//    red: u8,
//    green: u8,
//    blue:u8
//}




                                                            //Tuple Struct
//struct Color(u8,u8,u8)




                                                           //3rd Example struct with method
struct Person{
    first_name: String,
    last_name: String
    }
    impl Person {
        fn new(first: &str, last: &str) -> Person{
            Person{
                first_name:first.to_string(),
                last_name:last.to_string()
            }
        }

         //GET Full name                                      *example using same person struct*
         fn full_name(&self) -> String {                // self is similiar to "this" in other languages
            format!("{} {}", self.first_name, self.last_name)
         }

        //SET Last name                                       *example using same person struct*
         fn set_last_name(&mut self, last: &str){
            self.last_name = last.to_string();
         }
         //Name to tuple                                      **example using same person struct**
         fn to_tuple(self) ->(String, String){
            (self.first_name, self.last_name)

         }
    }



// this line is used to seperate function declarations from the run function however both sides of this line work together 
/********************************************************************************************************************************************************************/


pub fn run(){
                                                               //~example using traditional struct
//    let mut c = Color{
//       red: 255,
//        green: 0,
//        blue :0
//    };
//    c.red =200;
//   println!("Color: {} {} {}", c.red, c.green, c.blue);




                                                                  //~example using tuple struct
 //   let mut x = Color(150,150,150);
 //   c.0 = 200;
 //   println!("Color: {} {} {}", c.0, c.1, c.2);
              
  


                                                                  //~example using person struct
    let mut p = Person::new("John", "Doe");
    println!("The name together is {}", p.full_name());

    println!("Person(calling F&L seperately) : {} {}", p.first_name, p.last_name);



                                                                 //~example 2 using person struct
    println!(".....new example call.....");
    p.set_last_name("Williams");
    println!("Name changed to: {}", p.full_name());




                                                                  //~example 3 using person struct
    println!(".....new example call.....");
    println!("Person Tuple {:?}", p.to_tuple());


}