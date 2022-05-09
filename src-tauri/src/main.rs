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

#[macro_use]
extern crate lazy_static;
lazy_static! {
  static   ref   CHILD:Mutex<Child> = Mutex::new(Command::new("nix")
 .arg("repl")
 .stdin(Stdio::piped())
 .stdout(Stdio::piped())
 .spawn()
 .expect("failed to execute child"));
 static   ref   STDIN:Mutex<ChildStdin> = Mutex::new( CHILD.lock().unwrap().stdin.take().unwrap());
//  static   ref   STDOUT:Mutex<ChildStdout> = Mutex::new( CHILD.lock().unwrap().stdout.take().unwrap());
}


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


#[tauri::command]
fn repl(payload:String) -> String  {
   
    STDIN.lock().unwrap().write_all(b"builtins.toJSON (builtins.attrNames config.users.users)\n").unwrap();
  

 
    format!("wait for response")
    
    
  }


fn main() {
  STDIN.lock().unwrap().write_all(b":l <nixpkgs/nixos>\n").unwrap();
  let mut out = BufReader::new(CHILD.lock().unwrap().stdout.take().unwrap());
  
  std::thread::spawn(move ||{


      println!("looping ");
      
       out.lines().for_each(|line|{
                if line.as_ref().unwrap()==""{return}
          println!("out: {}", line.as_ref().unwrap());
          std::fs::write("./src/bla.txt", line.unwrap()).expect("could not write to configuration");
     }
      );
 

  });

  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![get_config,save_config,repl])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
