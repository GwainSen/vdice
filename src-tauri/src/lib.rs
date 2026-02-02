// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod json_data;

#[tauri::command]
fn custom_console_log(msg: String){
    println!("{}",msg);
}

#[tauri::command]
fn my_custom_command(dice: String) -> String {
    json_data::read_simple_dice();
    if dice == "D2" {
        "2".into()
    }
    else {
        "10".into()
    }
   
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![custom_console_log, my_custom_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

