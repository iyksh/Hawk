// sudo env "PATH=$HOME/.cargo/bin:$PATH" cargo run

mod utils;
mod tools;
mod table;

use eframe::egui;


use std::{
    thread, 
    fs::OpenOptions, 
    io::Write, 
};


use table::table::TableEntry;

fn main() {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Hawk - Network Analyzer",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    );
}

#[derive(Default)]
pub struct MyApp {
    counter: i32,
    data: Vec<TableEntry>,
    is_running: bool,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {}
                    if ui.button("Save").clicked() {}
                });

                if ui.button("Run").clicked() {
                    let _ = OpenOptions::new()
                    .write(true)      // Open in write mode
                    .truncate(true)   // Truncate the file to zero length
                    .open("src/tools/atomic.txt")
                    .expect("file not found")
                    .write(b"1");
                    
                    self.is_running = true;
                    thread::spawn(|| {
                        tools::functions::start_sniffing();
                    });
                }

                if ui.button("Stop").clicked() {
                    let _ = OpenOptions::new()
                    .write(true)      // Open in write mode
                    .truncate(true)   // Truncate the file to zero length
                    .open("src/tools/atomic.txt")
                    .expect("file not found")
                    .write(b"0");
                
                    self.is_running = false;
                }

                if ui.button("Clear").clicked() {
                    self.data.clear();
                    self.counter = 0;
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Packet Information Table");
        
            // Wrap the grid in a ScrollArea
            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::Grid::new("packet_table")
                    .striped(true)
                    .min_col_width(100.0)
                    .show(ui, |ui| {
                        ui.label("Dest MAC");
                        ui.label("Src MAC");
                        ui.label("Src IP");
                        ui.label("Dest IP");
                        ui.label("Port");
                        ui.label("Protocol");
                        ui.label("Payload Length");
                        ui.end_row();
        
                        for entry in &self.data {
                            ui.label(&entry.dest_mac);
                            ui.label(&entry.src_mac);
                            ui.label(&entry.src_ip);
                            ui.label(&entry.dest_ip);
                            ui.label(entry.port.to_string());
                            ui.label(&entry.protocol);
                            ui.label(entry.payload_length.to_string());
                            ui.end_row();
                        }
                    });
            });
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.checkbox(&mut self.is_running, "Is running ?");
        
            if self.is_running {
                self.show_results();
            }


        });
        
    }
}

impl MyApp {
    fn insert_entry(&mut self, entry: TableEntry) {
        self.data.push(entry);
        self.counter += 1;
    }

    fn show_results(&mut self) {
            let entries = TableEntry::read_table_entries("src/tools/packages.txt");
            for entry in entries {
                if !self.data.contains(&entry) {
                    self.insert_entry(entry);
    
    
                }
            } 
        }
}

