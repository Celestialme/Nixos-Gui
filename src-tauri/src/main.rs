#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
mod nix_env;
mod worker;
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
fn get_packages() -> String{
  match std::fs::read_to_string("/etc/NIX_GUI/packages.json"){
    Ok(txt) => txt,
    Err(err) => "{\"error\":\"file not found\" }".to_string()
  }
}
#[tauri::command]
fn get_options() -> String{
  match std::fs::read_to_string("/etc/NIX_GUI/options.json"){
    Ok(txt) => txt,
    Err(err) => "{\"error\":\"file not found\" }".to_string()
  }
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
     
    nix_env::download(payload,window);



      "started download".into()
  }

  #[tauri::command]
  fn update_db(window:Window) -> String{

     
    nix_env::update_packages(window)

  }


#[tauri::command]
fn save_config(payload:String) -> String{
   if !nix_env::is_root(){
     return "denied".into()
   }
    std::fs::write("/etc/nixos/configuration.nix", payload.to_string()).expect("could not write to configuration");
    "success".into()
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

#[tauri::command]
fn get_nix_env_pkgs()->Vec<std::string::String>{
  nix_env::get_nix_env_pkgs()
}

#[tauri::command]
fn get_channels()->Vec<std::string::String>{
  nix_env::get_channels()
}
#[tauri::command]
fn add_channel(name:String,url:String)->Vec<std::string::String>{
  nix_env::add_channel(name,url)
}
#[tauri::command]
 fn remove_channel(name:String)->Vec<std::string::String>{
  nix_env::remove_channel(name)
}
#[tauri::command]
 fn update_channels()->String{
  nix_env::update_channels()
}

#[tauri::command]
 fn get_generations()->Vec<String>{
  nix_env::get_generations()
}

#[tauri::command]
 fn rebuild_switch(window:Window)->String{
  nix_env::rebuild_switch(window)
}

#[tauri::command]
 fn exp(){
  worker::filter("google",vec!["nixos.googler".to_owned(), "nixos.htop".to_owned(), "nixos.google-clasp".to_owned(), "nixos.steam".to_owned(),"nixos.google-chrome".to_owned()]);
}

fn main() {
  STDIN.lock().unwrap().write_all(b":l <nixpkgs/nixos>\n").unwrap();
  


  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![get_packages,get_options,get_config,save_config,repl,start_repl,start_download,
    update_db,get_nix_env_pkgs,get_channels,add_channel,remove_channel,update_channels,get_generations,rebuild_switch,exp
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
