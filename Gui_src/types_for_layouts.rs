#![allow(non_camel_case_types)]
#![allow(unreachable_code)]
use egui::Id;
use egui::util::id_type_map::*;

/**
 * Very similar to custom_types which is the main sheibang. but used for the layout.rs module to avoid conflicting implementations of eframe...
 */

/********************************************App Struct*************************************************************/
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct App2 {
    pub name: String,
    pub age: i32,
    // pub on_opt: Radio_option,
    // pub off_opt: Radio_option,
    // pub Commands: Vec<String>,
    // pub on_group: Vec<Radio_option>,
    // pub off_group: Vec<Radio_option>,
}

impl App2 {
    pub fn new () -> Self {
        Self::default()
    }

    // pub fn getEm() -> App2 {
    //     Self { Commands: ["This".to_string(), "That".to_string(), "Those".to_string()].to_vec(), ..Default::default()}
    // }

}

impl Default for App2 {
    fn default() -> App2 {
        Self { 
            name: "Joe NoName".to_string(),
            age: 99,
            // on_opt: Radio_option::Selected,
            // off_opt: Radio_option::Not_Selected,
            // on_group: [Radio_option::Selected, Radio_option::Not_Selected, Radio_option::Selected].to_vec(),
            // off_group: [Radio_option::Not_Selected, Radio_option::Selected, Radio_option::Not_Selected].to_vec(),
            // Commands: ["nil".to_string(), "null".to_string(), "nincompoop".to_string()].to_vec(),
        }
    }
}

/**************************************Enums*************************************************/
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Radio_option { Not_Selected, Selected}
