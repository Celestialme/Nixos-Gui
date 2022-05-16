#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use std::io::{BufRead, BufReader, Read};
use std::process::{Command, Stdio,Child,ChildStdin,ChildStdout};
use std::sync::{Arc,Mutex};
use std::io::Write;
use std::{thread, vec};
use serde::{Serialize, Deserialize};
use tauri::{Manager, Window};
use strip_ansi_escapes;

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
static ref IS_RUNING:Mutex<bool> = Mutex::new(false);
}

 
#[tauri::command]
fn get_config() -> String  {

  let text =  std::fs::read_to_string("/etc/nixos/configuration.nix").expect("configuration file not found");

    let ast = rnix::parse(&text);
    
    let configuration =serde_json::to_string(&ast.node()).unwrap();
    
    configuration
   
  }

  #[tauri::command]
  fn start_download(payload:String,window:Window) -> String{
      println!("downloading... {}",payload.to_string());
      let mut child = Command::new("nix")
      .arg("build")
      .arg("--no-link")
      .arg("-f")
      .arg("<nixpkgs>")
      .arg(payload.to_string())
      .stdin(Stdio::piped())
      .stdout(Stdio::piped())
      .stderr(Stdio::piped())
      .spawn()
      .expect("failed to execute child");


      let mut out = BufReader::new(child.stderr.take().unwrap());
      std::thread::spawn(move ||{
     
        out.lines().for_each(|line|{
          println!("out::");
       if line.as_ref().unwrap()==""{return}
           println!("out: {}", line.as_ref().unwrap());

          //  window.emit("repl", std::str::from_utf8(&strip_ansi_escapes::strip(line.as_ref().unwrap()).unwrap()).unwrap()).unwrap();
    
      }
       );
    
    
    });













      "started download".into()
  }

#[tauri::command]
fn save_config(payload:String) -> String{
    std::fs::write("/etc/nixos/configuration.nix", payload.to_string()).expect("could not write to configuration");
    "saved".into()
}


#[tauri::command]
fn repl(payload:String) -> String  {
  //  builtins.toJSON (builtins.attrNames config.users.users)
    let data = format!("{}\n",payload);
    STDIN.lock().unwrap().write_all(data.as_bytes()).unwrap();
  

 
    format!("wait for response")
    
    
  }
#[tauri::command]
fn start_repl(window: Window){
  if *IS_RUNING.lock().unwrap(){
    return
  }
  *IS_RUNING.lock().unwrap()=true;
  let mut out = BufReader::new(CHILD.lock().unwrap().stdout.take().unwrap());

  std::thread::spawn(move ||{
     
    out.lines().for_each(|line|{
   if line.as_ref().unwrap()==""{return}
       println!("out: {}", line.as_ref().unwrap());
       
       // std::fs::write("./src/bla.txt", line.unwrap()).expect("could not write to configuration");
       window.emit("repl", std::str::from_utf8(&strip_ansi_escapes::strip(line.as_ref().unwrap()).unwrap()).unwrap()).unwrap();

  }
   );


});
}

fn main() {
  STDIN.lock().unwrap().write_all(b":l <nixpkgs/nixos>\n").unwrap();
  


  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![get_config,save_config,repl,start_repl,start_download])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
