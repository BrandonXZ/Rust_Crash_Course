#![allow(unused_assignments)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(unused_mut)]
#![allow(dead_code)]    //These are pre-linter directives
/**
 * This module gives a correct and working example of using radio buttons for GUI interfaces. 
 * Any files with _rb appended to the filename are support files for this example only.
 * what should be a relatively simple thing, is made difficult by inaccurate naming and less than favorable solution approaches
 * than what is possible with rust. This is compounded by piss-poor documentation. So this example (hopefully) will simplify using 
 * Radio buttons and other Gui widgets (ui elements)
 * 
 */

    //Internal Dep Crates
mod custom_types;
use custom_types::{ App,  Radio_option };
    //External Dep Crates
use eframe::{ egui, NativeOptions };


/****************************************************Main Flow********************************************* */

fn main() {
    
    let mut app = App::new();                           //(1) create an instance of a struct to track elements/changes.

    let native_options = NativeOptions::default();      //Pre-requisite step required for working with eframe used for creating frame options
    
    eframe::run_native("Radio Button how-to",           //Pre-requisite step, this is calling the run the actual eframe function
    native_options,  
    Box::new(|_cc| Box::new(app)));
}


/****************************************************Function Definitions/Impl********************************************* */


impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {                                      //BP code
        egui::CentralPanel::default().show(ctx, |ui| {                                                          //BP code

            ui.horizontal ( |ui| {                      //(2) We create a horizontal(any) layout first
                if ui.radio_value(&mut self.on_opt, Radio_option::Selected, "A").clicked() { //(3a) Next, we fill it with widgets 
                    self.off_opt = Radio_option::Not_Selected;   
                    self.on_opt = Radio_option::Selected;
                    println!("A clicked");
                };

                if ui.radio_value(&mut self.off_opt, Radio_option::Selected, "B").clicked() { //(3b) Note the alternating behavior
                    self.on_opt = Radio_option::Not_Selected;
                    self.off_opt = Radio_option::Selected;
                    println!("B clicked");
                };

            }); //Horizontal layout

        }); //central panel

    } //update function for egui

} //impl app




/********************************************************************Code Notes***********************************************************************************/

/* 

**Consider This | ------>          let radio_on = ui.radio_value(current_value, alternative, text);

    I finally have the radio buttons working in a way I can understand. 
    "alternative" is a piss poor name for the function arg of radio_value. This should be "on_option"
    "truthy-option", or even better "option_when_selected_&_circle_is_filled"... -.-'
    
    
    



*/