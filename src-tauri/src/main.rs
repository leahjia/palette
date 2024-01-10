// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use colorgrad::{Color, CustomGradient};
use palette::{FromColor, Lch, Srgb};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate_gradient])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn generate_gradient(r: u8, g: u8, b: u8) -> Vec<Vec<u8>> {
    println!("Yo I got something: {}, {}, {}", r, g, b);
    let my_rgb = Srgb::new(r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0);

    let my_lch = Lch::from_color(my_rgb.into_linear());
    dbg!(my_lch);

    let gradient = CustomGradient::new()
        .colors(&[
            Color::from_rgb(0.0, my_lch.chroma, my_lch.hue.into()),
            Color::from_rgb(my_lch.l, my_lch.chroma, my_lch.hue.into()),
            Color::from_rgb(128.0, my_lch.chroma, my_lch.hue.into()),
        ])
        .build()
        .unwrap();

    let colors = gradient.colors(10).into_iter().map(|c| vec![c.r as u8, c.g as u8, c.b as u8]).collect::<Vec<_>>();

    dbg!(&colors);
    colors
}
