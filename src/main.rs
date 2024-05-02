use macroquad::prelude::*;
use ratatui::{
    prelude::{Stylize, Terminal},
    widgets::{Paragraph,Wrap},
};
use ratframe::RataguiBackend;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;


#[derive(Debug, Serialize, Deserialize)]
pub struct Branch {
    pub name: String,
    pub commit: Commit,
}

#[macroquad::main("egui with macroquad")]
async fn main() {
    let boop = RataguiBackend::new(100, 50);
    let mut terminal = Terminal::new(boop).unwrap();

    let res = reqwest::Client::new()
    .get("https://api.github.com/repos/rustwasm/wasm-bindgen/branches/master")
    .header("Accept", "application/vnd.github.v3+json")
    .send()
    .await?;

let text = res.text().await?;
let branch_info: Branch = serde_json::from_str(&text).unwrap();

    
    loop {
        clear_background(WHITE);

        // Process keys, mouse etc.

        terminal
        .draw(|frame| {
            let area = frame.size();
            frame.render_widget(Paragraph::new("Hello Rataguiii and hello macroquad yayyyy weeee ").white().on_blue().wrap(Wrap { trim: false }), area);
        })
        .expect("epic fail");
    

        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("egui ❤ macroquad")
                .show(egui_ctx, |ui| {
                    ui.add(terminal.backend_mut());
                });
        });

        // Draw things before egui

        egui_macroquad::draw();
        
        // Draw things after egui

        next_frame().await;
    }
}