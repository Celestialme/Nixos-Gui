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
 .stderr(Stdio::piped())
 .spawn()
 .expect("failed to execute child"));
 static   ref   STDIN:Mutex<ChildStdin> = Mutex::new( CHILD.lock().unwrap().stdin.take().unwrap());
//  static   ref   STDOUT:Mutex<ChildStdout> = Mutex::new( CHILD.lock().unwrap().stdout.take().unwrap());
static ref IS_RUNING:Mutex<bool> = Mutex::new(false);

static ref  RESPONSE:Arc<Mutex<nix_env::Value>> = Arc::new(Mutex::new(
  {nix_env::Value{
        status:"out".to_string(),
        value:"".to_string(),
    }}
));



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
fn repl_command(payload:String) -> String  {
    let data = format!("{}\n",payload);
    nix_env::repl_command(Arc::clone(&RESPONSE),&data,&STDIN.lock().unwrap())
  }



fn start_repl(){
 
  let mut out = BufReader::new(CHILD.lock().unwrap().stdout.take().unwrap());
  let mut err = BufReader::new(CHILD.lock().unwrap().stderr.take().unwrap());
  let response = Arc::clone(&RESPONSE);
  std::thread::spawn(move ||{
    out.lines().for_each(|line|{
      if line.as_ref().unwrap()==""  {return}
      
      (*response.lock().unwrap()).value =line.as_ref().unwrap().to_string();
      (*response.lock().unwrap()).status = "OUT".to_string();
      
    }
  );
  
  });


let response = Arc::clone(&RESPONSE);
std::thread::spawn(move ||{
  err.lines().for_each(|line|{
    if line.as_ref().unwrap()=="" || line.as_ref().unwrap().starts_with("warning: Nix") || line.as_ref().unwrap().contains("Inappropriate ioctl for device"){return}
     (*response.lock().unwrap()).value =line.as_ref().unwrap().to_string();
     (*response.lock().unwrap()).status = "ERROR".to_string();
     println!("{}",line.as_ref().unwrap());
    
  }
);

});

}

#[tauri::command]
fn get_nix_env_pkgs()->Vec<std::string::String>{ //returns keys nixos.htop
  let tmp = nix_env::get_nix_env_pkgs().into_iter().map(|pkg_name| {
    let mut res="";
    for (key, val) in worker::PKGS.as_object().unwrap().iter() {
      if !val["name"].is_null() && val["name"].as_str().unwrap().to_string().trim()==pkg_name.trim() {
        res = key;
        break
      };
  };


  res.to_owned()

  }).collect();

  tmp
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
 fn filter_packages(window:Window,value:String,keys:Vec<String>){
  *(worker::CURRENT_VALUE.lock().unwrap()) = value.to_owned();
  std::thread::spawn(move||{
    let result = worker::filter(&value,keys);
    if *(worker::CURRENT_VALUE.lock().unwrap())!= value{
      return
    }
    window.emit("filterPackages",result);
    
  });
}

#[tauri::command]
fn filter_dict(window:Window,filter_key:String){
  *(worker::CURRENT_VALUE.lock().unwrap()) = filter_key.to_owned();
  std::thread::spawn(move||{
  worker::filter_dict(window,&filter_key,&repl_command);
  });
}

#[tauri::command]
fn filter_options(window:Window,value:String){

  *(worker::CURRENT_VALUE.lock().unwrap()) = value.to_owned();
  std::thread::spawn(move||{
    worker::filter_options(window,value);
  });


}

fn main() {
  STDIN.lock().unwrap().write_all(b":l <nixpkgs/nixos>\n").unwrap();
  start_repl();


  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![get_packages,get_options,get_config,save_config,repl_command,start_download,
    update_db,get_nix_env_pkgs,get_channels,add_channel,remove_channel,update_channels,get_generations,rebuild_switch,filter_packages,filter_dict,filter_options
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
