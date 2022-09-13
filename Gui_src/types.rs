/**
 * A dummied down example of looping through something to generate ui elements and referencing each of those elements (i.e their values,attributes, etc.) correctly
 */


pub struct MyApp {
    pub name: String,
    pub age: u32,
    pub members: Vec<Members>
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            members: vec![],
        }
    }
}

#[derive(Clone)]
pub struct Members {
    pub name: String, 
    pub age: u32, 
}

impl Default for Members {
    fn default() -> Self {
        Self {
            name: "Blank".to_string(),
            age: 99,
        }
    }
}

// impl Vector for Members {
//     fn to_vec() -> Self {
//         Self {

//         }
//     }
// }