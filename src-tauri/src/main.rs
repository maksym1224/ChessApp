#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use app::chess;
use palette::{FromColor, Gradient, Lch, Srgb};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            generate_gradient,
            hover_square,
            unhover_square,
            drop_square,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn hover_square(square: &str) {
    dbg!(square);
}

#[tauri::command]
fn unhover_square(square: &str) {
    dbg!(square);
}

#[tauri::command]
fn drop_square(source_square: &str, target_square: &str, piece: &str) {
    dbg!(source_square, target_square, piece);
}

#[tauri::command]
fn generate_gradient(r: u8, g: u8, b: u8) -> Vec<Vec<u8>> {
    println!("invokes {} {} {}", r, g, b);
    let my_rgb = Srgb::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);
    let my_lch = Lch::from_color(my_rgb.into_linear());
    dbg!(my_lch);
    let gradient = Gradient::new(vec![
        Lch::new(0.0, my_lch.chroma, my_lch.hue),
        my_lch,
        Lch::new(128.0, my_lch.chroma, my_lch.hue),
    ]);
    let colors = gradient
        .take(10)
        .map(|color| {
            let (r, g, b) = Srgb::from_color(color).into_components();
            vec![(r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8]
        })
        .collect::<Vec<_>>();
    dbg!(&colors);
    colors
}
