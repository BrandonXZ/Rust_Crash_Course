#![allow(unused_assignments)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(unused_mut)]
#![allow(dead_code)]
//Rust Utils
use std::{ process::exit, any::Any };
//external Dep Crates
use eframe::{ egui, NativeOptions };
use egui::{ Context, Ui, Id, style, WidgetInfo, Widget, util::IdTypeMap };
use egui::util::cache::{CacheStorage, ComputerMut, FrameCache};
use crate::egui::Align::Center;
///Internal Dep Crates
use crate::types_for_layouts::{ App2,  Radio_option };


pub fn main() {

    // create an instance of a struct to track elements/changes.
    let mut app = App2::new();

    //create frame options
    let native_options = NativeOptions::default();

    //run the actual frame
    eframe::run_native("Radio Button how-to", 
    native_options,  
    Box::new(|_cc| Box::new(app)));
}

impl eframe::App for App2 {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("                My egui Application");
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));

            ui.add_space(15.5);
            ui.heading("                Lets see that laid out horizontally");
            ui.add_space(15.5);

            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
                ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
                if ui.button("Click each year").clicked() {
                    self.age += 1;
                }
            });
            // egui::Window::hscroll(self, true);
            // egui::Window::vscroll(self, true);
            
            ui.add_space(15.5);
            ui.heading("                    How about Vertically?");
            ui.add_space(15.5);

            ui.vertical(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
                ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
                if ui.button("Click each year").clicked() {
                    self.age += 1;
                }

            });
            ui.add_space(15.5);
            ui.heading("                A Challenge.................scrollable ~.~' ");
            ui.add_space(15.5);

            egui::ScrollArea::vertical().show(ui, |ui| {
                // Add a lot of widgets here.
            Ui::with_layout(ui, egui::Layout::with_cross_justify(egui::Layout::top_down(Center), true), |ui|{ 
                ui.vertical(|ui| {
                    ui.label("Your name: ");
                    ui.text_edit_singleline(&mut self.name);
                    ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
                    if ui.button("Click each year").clicked() {
                        self.age += 1;
                    }
    
                });
            });


            ui.add_space(15.5);
            ui.heading("                Now, Make each element fill the space....");
            ui.add_space(35.5);

            ui.centered_and_justified(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
                ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            });
            ui.centered_and_justified(|ui| {
                if ui.button("Click each year").clicked() {
                    self.age += 1;
                }
            }); 
            });


        }); //central panel
    }
} //impl app

/********************************************************************Code Notes***********************************************************************************/

/*finally have the radio buttons working in a way I can understand. 
    "alternative" is a piss poor name for the function arg of radio_value. should be "on_option"
    "truthy-option", "activeoption" or even better "option_when_selected_&_circle_is_filled"... -.-'*/

