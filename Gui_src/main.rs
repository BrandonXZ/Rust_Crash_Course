#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(dead_code)]
#![allow(non_snake_case)]
/**
 * This is the staring point for the GUI-series instructionals. A Crash course designed to simplify all thing gui for native console apps, 
 * those intended for web facing applications(via WASM) and those designed to be accessible from both. 
 * Not sure if this will cover embedded sys gui's just yet as this requires a significant effort and advanced rust knowledge to implement and debug... 
 */

mod types;

use eframe::egui;

fn main() { 
    let options = eframe::NativeOptions::default();
    let mut member_vec = vec![];
    let name1 = types::Members { name:"Alpha".to_string(), age: 16}; 
    let name2 = types::Members { name: "Bravo".to_string(), age: 17};
    let name3 = types::Members { name: "Charlie".to_string(), age: 18};
    member_vec.push(name1); member_vec.push(name2); member_vec.push(name3);


    let app_instance = types::MyApp { members: member_vec, ..Default::default() }; 
    eframe::run_native(
        "My UI using EGUI Example",
        options,
        Box::new(|_cc| Box::new(app_instance)),
    );
}


impl eframe::App for types::MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");

            for mut i in self.members.clone() {
                ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut i.name);
            });
            ui.add(egui::Slider::new(&mut i.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                i.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", i.name, i.age));
            }
            
        });
    }
}