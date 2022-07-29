use core::Core;
use std::iter::from_fn;
use eframe::{egui::CentralPanel, run_native, App, NativeOptions};

#[derive(Default)]
struct Atlantis {
    core: Core,
}

impl Atlantis {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    fn get_hex(&self, arr: &[u8], size: usize) -> String {
        let mut s = String::new();
        for i in 0..arr.len() {
            s.push_str(&format!("{:02x} ", arr[i]));
        }

        if arr.len() < size {
            for _ in arr.len()..size {
                s.push_str("   ");
            }
        }

        s
    }

    fn get_ascii(&self, arr: &[u8], size: usize) -> String {
        let raw: Vec<String> = arr
            .iter()
            .map(|num| {
                if *num >= 32 && *num <= 126 {
                    (*num as char).to_string()
                }
                else {
                    '.'.to_string()
                }
            })
            .collect();

        let mut s = raw.join(" ");

        if arr.len() < size {
            for _ in arr.len()..size {
                s.push_str("  ");
            }
        }

        s
    }

    fn hex_dump(&self, arr: &[u8], size: usize) -> impl Iterator<Item = String> {
        let mut s = String::new();
        let mut i = 0;
        let chucks = arr.chunks(size);

        for chunk in chucks {
            let hex = self.get_hex(chunk, size);
            let ascii = self.get_ascii(chunk, size);
            
            s.push_str(&format!("{:08x}  ", i));
            s.push_str(&hex);
            s.push_str(&format!("  {}\n", ascii));
            i += size;
        }

        from_fn(move || {
            if s.is_empty() {
                None
            }
            else {
                let line = s.clone();
                s.clear();
                Some(line)
            }
        })
    }
}

impl App for Atlantis {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            let arr = [
                104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 0, 0, 0, 0, 0,
                104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 0, 0, 0, 0, 0,
                104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 0, 0, 0, 0, 0,
            ];

            for lines in self.hex_dump(&arr, 16) {
                ui.code(lines);
            }
        });
    }    
}

fn main() {
    let win_options = NativeOptions::default();
    run_native("GBA", win_options, Box::new(|cc| Box::new(Atlantis::new(cc))));
}
