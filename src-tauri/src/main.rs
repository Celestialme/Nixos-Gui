#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use std::io::{BufRead, BufReader, Read};
use std::process::{Command, Stdio,Child,ChildStdin,ChildStdout};
use std::sync::{Mutex};
use std::io::Write;
use std::{thread, vec};
use serde::{Serialize, Deserialize};




#[tauri::command]
fn get_config() -> String  {

  let text =  std::fs::read_to_string("/etc/nixos/configuration.nix").expect("configuration file not found");

    let ast = rnix::parse(&text);
    
    let configuration =serde_json::to_string(&ast.node()).unwrap();
    
    configuration
   
  }



#[tauri::command]
fn save_config(payload:String) -> String{
    std::fs::write("/etc/nixos/configuration.nix", payload.to_string()).expect("could not write to configuration");
    "saved".into()
}




fn main() {
  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![get_config,save_config])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
