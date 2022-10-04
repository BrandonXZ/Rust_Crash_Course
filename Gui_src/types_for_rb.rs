/**
 * just as a good coding convention, I like to separate custom types associated with the program from the actual logic/flow coding used by the program
 * I personally feel like this contributes to cleaner, easier to read code.
 */

/********************************************App Struct*************************************************************/
#[derive(Debug, Clone, PartialEq, Eq)]              //technically only Debug, and partialEq are required for this example, which allow for printing instance vals
pub struct App {                                    //and comparing values internally/on the fly respectively 
    pub onOpt_id: Id,
    pub offOpt_id: Id,
    pub on_opt: Radio_option,
    pub off_opt: Radio_option,
    pub tog_current: bool,
}

impl App {                                              //Here we use impl to give our struct functions(methods), this just makes it easier/less verbose 
    pub fn new () -> Self {                             //to create a new instance of the struct
        Self::default()
    }
}

impl Default for App {                                  //here we do the same thing as above, only with a function(method) called default. 
    fn default() -> App {                               //Its worth noting we could easily use #[derive(default)] to accomplish the same thing but this is explicitly
        Self {                                          //done because in a prev version of this tutorial I was messing with widget ID's which requires a hashed val
            on_opt: Radio_option::Selected,
            off_opt: Radio_option::Not_Selected,
            onOpt_id: egui::Id::new("Default"),
            offOpt_id: egui::Id::new("Default"),
            tog_current: true,
        }
    }
}

/**************************************Enums*************************************************/
#[derive(PartialEq, Eq, Debug, Clone, Copy)]                //Remember, Enums are data types defined when only 1 value is possible at any given time. either this or that
pub enum Radio_option { Not_Selected, Selected}