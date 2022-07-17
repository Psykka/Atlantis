use eframe::{run_native, App, egui::CentralPanel, NativeOptions};
use core::Core;

#[derive(Default)]
struct GBA {
    core: Core,
}

impl GBA {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    fn get_hex(&self, arr: &[u8]) -> String {
        let mut s = String::new();
        for i in 0..arr.len() {
            s.push_str(&format!("{:02x} ", arr[i]));
        }
        s
    }

    fn get_ascii(&self, arr: &[u8]) -> String {
        let build_string_vec: Vec<String> = arr.iter().map(|num| {
            if *num >= 32 && *num <= 126 { (*num as char).to_string() }
            else { '.'.to_string() }
        }).collect();
    
        build_string_vec.join(" ")
    }

    fn hex_dump(&self, arr: &[u8], buff: usize) -> Vec<String> {
        if arr.len() < 16 {
            panic!("Array is too small to dump");
        }

        let mut hex_dump: Vec<String> = Vec::new();
        let mut i = 0;

        while i < arr.len() {
            let mut s = String::new();
            let dump = &arr[i..i+buff];
            s.push_str(&format!("0x{:08x}: ", i));
            s.push_str(&self.get_hex(dump));
            s.push_str(&format!("  {}", self.get_ascii(dump)));
            hex_dump.push(s);
            i += buff;
        }

        hex_dump
    }
}

impl App for GBA {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            let arr = &[104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 0, 0, 0, 0, 0];
            for line in self.hex_dump(arr, 8) {
                ui.label(line);
            }
        });
    }
}

fn main() {
    let win_options = NativeOptions::default();
    run_native("GBA", win_options, Box::new(|cc| Box::new(GBA::new(cc))));
}