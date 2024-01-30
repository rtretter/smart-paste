#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use egui::IconData;
use enigo::{
    Enigo, Key, Keyboard, Settings,
    Direction::Click,
};
use global_hotkey::{GlobalHotKeyManager, hotkey::{HotKey, Modifiers, Code}, GlobalHotKeyEvent, HotKeyState};
use arboard::Clipboard;
use eframe::egui;
use std::{thread, time::Duration};

fn main() {
    let manager = GlobalHotKeyManager::new().unwrap();
    let hotkey_paste = HotKey::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyV);
    manager.register(hotkey_paste).unwrap();
    let global_hotkey_channel = GlobalHotKeyEvent::receiver();

    std::thread::spawn(move || loop {
            thread::sleep(Duration::from_millis(100));
            if let Ok(event) = global_hotkey_channel.try_recv() {
                if hotkey_paste.id() == event.id && event.state == HotKeyState::Released {
                    let mut clipboard = Clipboard::new().unwrap();
                    
                    let mut settings = Settings::default();
                    settings.mac_delay = 1;
                    let mut enigo = Enigo::new(&settings).unwrap();
                    let _ = enigo.key(Key::Control, Click);
                    thread::sleep(Duration::from_millis(50));
                    let clipboard_text = clipboard.get_text().unwrap_or_else(|_| {
                        println!("Clipboard is empty!");
                        "".to_owned()
                    }).replace("\r", "");
                    for (index, line) in clipboard_text.split("\n").enumerate() {
                        if index > 0 {
                            let _ = enigo.key(Key::Return, Click);
                        }
                        let _ = enigo.text(line);
                        println!("{}: {}", index, line);
                    }
                }
            }
    });
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_icon(load_icon()).with_decorations(false).with_transparent(true).with_mouse_passthrough(true),
        ..Default::default()
    };
    let _ = eframe::run_native(
        "smart-paste",
        options,
        Box::new(|_cc| Box::<SmartPaste>::default()),
    );
}

fn load_icon() -> IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let icon = include_bytes!("./../assets/icon.png");
        let image = image::load_from_memory(icon)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}

struct SmartPaste{}
impl Default for SmartPaste {
    fn default() -> Self {
        Self{}
    }
}
impl eframe::App for SmartPaste {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_rgba_unmultiplied()
    }

    fn update(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
    }
}
