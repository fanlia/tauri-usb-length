#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[tauri::command]
fn greet(name: &str) -> String {
   format!("Hello, {}!", name)
}

#[tauri::command]
fn usb() -> usize {
    let mut i = 0;
    for _device in rusb::devices().unwrap().iter() {
        i += 1;
    }
    i
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet])
    .invoke_handler(tauri::generate_handler![usb])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
