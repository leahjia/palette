// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use palette::{FromColor, Lch, Srgb};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate_gradient])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn generate_gradient(r: u8, g: u8, b: u8) {
    println!("Yo I got something: {}, {}, {}", r, g, b);
    let rgb = Srgb::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);
    // let lch: Lch = Lch::from_color(rgb.into_linear());
    // let gradient = Gradient::new(vec![Lch::new(0, lch.chroma as i32, lch.hue), lch, Lch::new(1, lch.chroma as i32, lch.hue)]);
}
