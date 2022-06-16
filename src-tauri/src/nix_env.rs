// fn main() {
//  let content =  std::fs::read_to_string("./configuration.nix").expect("configuration file not found");

//   let ast = rnix::parse("{a = b;}");

//    serde_json::to_writer(std::io::stdout(), &ast.node()).unwrap();

// }
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio,ChildStdout,ChildStdin};
use std::io::{Read,Write};
use tauri::{Manager, Window};
use std::fs::File;  
use std::{thread, time};
use std::sync::{Arc,Mutex};
struct Value{
  status:String,
  value:String
}

pub fn is_root()->bool{
   
let p = Command::new("whoami")

.output()
.expect("failed to execute child");


std::str::from_utf8(&p.stdout).unwrap().trim()=="root"
}

pub fn download(app:String,window:Window){


   let mut child = Command::new("nix-env").arg("-iA").arg("nixos.".to_owned()+&app)
  //let mut child = Command::new("nix-env").arg("-iA").arg("-f").arg("/nixos-unstable").arg(&app).args(["--option","sandbox","false"])
  .stdin(Stdio::piped())
  .stdout(Stdio::piped())
  .stderr(Stdio::piped())
  .spawn()
  .expect("failed to execute child");
// let mut stdin = child.stdin.take().expect("failed to get stdin");
let  output = child.stderr.take().unwrap();  
let out = BufReader::new(output);
std::thread::spawn(move ||{
let mut todo:Vec<String> = Vec::new();
let mut todo_max_length = 0;
let mut success = "true";
let mut error_msg = String::new();
window.emit(&format!("{}-{}","progress",app.replace(".","")), format!("{{ \"progress\":[{},{}],\"msg\":\"{}\" }}",0,1,"")).unwrap();
out.lines().for_each(|line|{
  let mut line = line.unwrap();  
 line = line.trim().to_string();
 println!("{}",line);
      if line==""{return}
      if line.contains("error") || success =="false"{
          success = "false";    
          error_msg+=&(r"<br>".to_owned()+&line);
          
          return
      };
      if line.starts_with("/nix/store"){
          todo.push(line);
          todo_max_length+=1;
      } else{
          match todo.iter().position(|r| line.contains(r)) {
            None => "None",
            Some(val) => {
              todo.remove(val);
            "Removed"
            },
        };
        
        window.emit(&format!("{}-{}","progress",app.replace(".","")), format!("{{ \"progress\":[{},{}],\"msg\":\"{}\" }}",todo_max_length-todo.len(),todo_max_length,
        line
        .replace("\"","'")
        .replace(r"`","'")
        .replace(".",".<wbr>")
        .replace("_","_<wbr>")
        .replace("-","_<wbr>")
        .replace("/","/<wbr>")
      )).unwrap();
      }
 // println!("{}/{}",build_length-build_list.len(),build_length)
 });

window.emit(&format!("{}-{}","progress",app.replace(".","")), format!("{{ \"progress\":[{},{}],\"msg\":\"{}\" }}",1,1,"")).unwrap();
window.emit(&format!("{}-{}","finish",app.replace(".","")), success).unwrap();
if success == "false"{
  println!("eroooooooooooooooooooooooor {}",error_msg);
window.emit(&format!("{}-{}","progress",app.replace(".","")), format!("{{ \"progress\":[{},{}],\"msg\":\"{}\" }}",todo_max_length-todo.len(),todo_max_length,
        error_msg
        .replace("\"","'")
        .replace(r"`","'")
        .replace(".",".<wbr>")
        .replace("_","_<wbr>")
        .replace("-","_<wbr>")
        .replace("/","/<wbr>")
      )).unwrap();
    };
 println!("finished");
});


}



pub fn update_packages(window:Window)->String{
  let  RESPONSE:Arc<Mutex<Value>> = Arc::new(Mutex::new(
    {Value{
          status:"out".to_string(),
          value:"".to_string(),
      }}
  ));
  let mut file:File;
  if !is_root(){
    return "denied".into();
  }
     Command::new("mkdir").arg("-p").arg("/etc/NIX_GUI").status().unwrap();
  File::create("/etc/NIX_GUI/packages.json").unwrap();
  file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("/etc/NIX_GUI/packages.json")
        .expect("packages.json not found");  
  
let mut child = Command::new("nix").arg("repl")
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .expect("failed to execute child");
let  stdin = child.stdin.take().expect("failed to get stdin");
let  stdout = child.stdout.take().unwrap();  
let  stderr = child.stderr.take().unwrap();
let err = BufReader::new(stderr);
let out = BufReader::new(stdout);

// println!("{:?}",std::env::current_dir());

let response = Arc::clone(&RESPONSE);
std::thread::spawn(move ||{
  err.lines().for_each(|line|{
    if line.as_ref().unwrap()=="" || line.as_ref().unwrap().starts_with("warning: Nix"){return}
     (*response.lock().unwrap()).value =line.as_ref().unwrap().to_string();
     (*response.lock().unwrap()).status = "ERROR".to_string();
    println!("{}",line.as_ref().unwrap());
    
  }
);

});


  
let response = Arc::clone(&RESPONSE);
std::thread::spawn(move ||{
  out.lines().for_each(|line|{
    if line.as_ref().unwrap()==""  {return}
    
    (*response.lock().unwrap()).value =line.as_ref().unwrap().to_string();
    (*response.lock().unwrap()).status = "OUT".to_string();
    
  }
);

});




repl_command(Arc::clone(&RESPONSE),"pkgs = import <nixpkgs> {}",&stdin);
thread::sleep(time::Duration::from_millis(1000));

std::thread::spawn(move ||{



let p = Command::new("nix-env").args(["-f","<nixpkgs>","-qaP","*","--no-name"])

    .output()
    .expect("failed to execute child");


let pkgs:Vec<String> = std::str::from_utf8(&p.stdout).unwrap().split("\n").map(|s| s.to_string()).collect();
// let pkgs = ["dart","firefox","wget","fish"]; // test data

let mut i = 0;
let length = pkgs.len();
let re_start = regex::Regex::new("^\\s*\"\\{").unwrap();
let re_end = regex::Regex::new("\\}\"$").unwrap();
for pkg in pkgs{
  i+=1;
 println!("{}/{}",i,length);
 let out =  repl_command(Arc::clone(&RESPONSE),&format!("let \
  try = builtins.tryEval; \
  pkg = pkgs.{}; \
  in builtins.toJSON (pkgs.lib.attrsets.setAttrByPath [\"{}\"] \
  {{ description=(try pkg.description or pkg.meta.description or \"\").value; \
  version=(try pkg.version or pkg.meta.version or \"\").value; \
  pname=(try pkg.pname or pkg.name or pkg.meta.name or \"\").value;\
  homepage = (try pkg.meta.homepage or \"\").value; }})",pkg,pkg),&stdin);
  let temp;
  if i==length{
    break
  }
  else if i==1{
    let mut tmp_string = out.replace(r"\\\\","").replace(r"\\\","¢").replace(r"\","").replace("¢",r"\");
    tmp_string = re_start.replace_all(&tmp_string,"").to_string();
    tmp_string = re_end.replace_all(&tmp_string,"").to_string();
    temp = format!("\n {{ {},",tmp_string);
    
  }else   if i==length-1{
    let mut tmp_string = out.replace(r"\\\\","").replace(r"\\\","¢").replace(r"\","").replace("¢",r"\");
    tmp_string = re_start.replace_all(&tmp_string,"").to_string();
    tmp_string = re_end.replace_all(&tmp_string,"").to_string();
     temp = format!("\n{} }}",tmp_string);
  }else{
    let mut tmp_string = out.replace(r"\\\\","").replace(r"\\\","¢").replace(r"\","").replace("¢",r"\");
    tmp_string = re_start.replace_all(&tmp_string,"").to_string();
    tmp_string = re_end.replace_all(&tmp_string,"").to_string();
     temp = format!("\n{},",tmp_string);
  }
 file.write_all(temp.as_bytes()).unwrap();
 window.emit(&format!("{}-{}","progress","update-db"), format!("{{ \"progress\":[{},{}],\"msg\":\"{}\" }}",i,length,"")).unwrap();

}
println!("finished");
window.emit(&format!("{}-{}","finish","update-db"), "true").unwrap();
});

let p = Command::new("nix-instantiate").args(["--eval","<nixpkgs/nixos/release.nix>","-A","options.drvPath"])
    .output()
    .expect("failed to execute child");
;
let  drv_path = std::str::from_utf8(&p.stdout).unwrap().replace("\"","").replace("\n","");

let p = Command::new("nix-store").args(["-r",&drv_path])
    .output()
    .expect("failed to execute child");

let mut options_json = std::str::from_utf8(&p.stdout).unwrap().replace("\n","").to_owned()+"/share/doc/nixos/options.json";
std::fs::copy(options_json, "/etc/NIX_GUI/options.json").expect("could not copy options json");

"success".into()
}


fn repl_command(value:Arc<Mutex<Value>>,command:&str,mut stdin:&ChildStdin)->String{
  (*value.lock().unwrap()).value = "NONE".to_string();
  let _command = format!("{}\n",command);
  stdin.write_all(_command.as_bytes()).unwrap();
loop{
  if value.lock().unwrap().value.to_string() != "NONE"   {
   
    break
  }
 
}

std::str::from_utf8(&strip_ansi_escapes::strip( value.lock().unwrap().value.to_string() ).unwrap()).unwrap().to_string()
}

pub fn get_nix_env_pkgs()->Vec<std::string::String>{


 
let p = Command::new("nix-env").args(["-q","--xml"])

    .output()
    .expect("failed to execute child");


let xml = std::str::from_utf8(&p.stdout).unwrap();
let re = regex::Regex::new("pname=\"(.*?)\"").unwrap();

let mut pkgs = Vec::new();
 for caps in re.captures_iter(xml) {
    pkgs.push(caps[1].to_string());
    }
pkgs
}

pub fn get_channels()->Vec<String>{
  
  let p = Command::new("nix-channel").arg("--list")

  .output()
  .expect("failed to execute child");

 std::str::from_utf8(&p.stdout).unwrap().split("\n").map(|s| s.to_string()).filter(|s| !s.is_empty())
.map(|s|{ 
let temp: Vec<&str> =  s.trim().split(" ").collect();
format!("{{\"name\":\"{}\", \"url\":\"{}\"}}",temp[0],temp[1])
}).collect()

}

pub fn add_channel(name:String,url:String)->Vec<std::string::String>{
  println!("{}  {}",url,name);
  let p = Command::new("nix-channel").args(["--add",&url,&name])

  .output()
  .expect("failed to execute child");
  get_channels()
}

pub fn remove_channel(name:String)->Vec<std::string::String>{
  
  let p = Command::new("nix-channel").args(["--remove",&name])

  .output()
  .expect("failed to execute child");
  get_channels()
}

pub fn update_channels()->String{
  
  let p = Command::new("nix-channel").arg("--update")

  .output()
  .expect("failed to execute child");
  println!("{}",std::str::from_utf8(&p.stderr).unwrap());
  if !std::str::from_utf8(&p.stderr).unwrap().contains("error"){
    "{\"success\":true}".into()
  }else{
    "{\"success\":false}".into()
  }
}


pub fn get_generations()->Vec<String>{
  
  let p = Command::new("nix-env").args(["--list-generations", "--profile", "/nix/var/nix/profiles/system"])

  .output()
  .expect("failed to execute child");

 std::str::from_utf8(&p.stdout).unwrap().split("\n").map(|s| s.to_string()).filter(|s| !s.is_empty())
.collect()

}


pub fn rebuild_switch(window:Window)->String{
  if !is_root(){
    return "denied".into()
  }
  let mut child = Command::new("nixos-rebuild").arg("switch").args(["--option", "sandbox" ,"false"])
  .stdin(Stdio::piped())
  .stdout(Stdio::piped())
  .stderr(Stdio::piped())
  .spawn()
  .expect("failed to execute child");
// let mut stdin = child.stdin.take().expect("failed to get stdin");
let  output = child.stderr.take().unwrap();  
let out = BufReader::new(output);
std::thread::spawn(move ||{
let mut todo:Vec<String> = Vec::new();
let mut todo_max_length = 0;
let mut success = "true";
let mut error_msg = String::new();
window.emit(&format!("{}-{}","progress","rebuild-switch"), format!("{{ \"progress\":[{},{}],\"msg\":\"{}\" }}",0,1,"")).unwrap();
out.lines().for_each(|line|{
  let mut line = line.unwrap();  
 line = line.trim().to_string();
 println!("{}",line);
      if line==""{return}
      if line.starts_with("error") || success =="false"{
          success = "false";    
          error_msg+=&(r"<br>".to_owned()+&line);
          
          return
      };
      if line.starts_with("/nix/store"){
          todo.push(line);
          todo_max_length+=1;
          return
      } else{
          match todo.iter().position(|r| line.contains(r)) {
            None => "None",
            Some(val) => {
              todo.remove(val);
            "Removed"
            },
        };
        
        window.emit(&format!("{}-{}","progress","rebuild-switch"), format!("{{ \"progress\":[{},{}],\"msg\":\"{}\" }}",todo_max_length-todo.len(),todo_max_length,
        line
        .replace("\"","'")
        .replace(r"`","'")
        .replace(".",".<wbr>")
        .replace("_","_<wbr>")
        .replace("-","_<wbr>")
        .replace("/","/<wbr>")
      )).unwrap();
      }
 // println!("{}/{}",build_length-build_list.len(),build_length)
 });

window.emit(&format!("{}-{}","progress","rebuild-switch"), format!("{{ \"progress\":[{},{}],\"msg\":\"{}\" }}",1,1,"")).unwrap();
window.emit(&format!("{}-{}","finish","rebuild-switch"), success).unwrap();
if success == "false"{
  println!("eroooooooooooooooooooooooor {}",error_msg);
window.emit(&format!("{}-{}","progress","rebuild-switch"), format!("{{ \"progress\":[{},{}],\"msg\":\"{}\" }}",todo_max_length-todo.len(),todo_max_length,
        error_msg
        .replace("\"","'")
        .replace(r"`","'")
        .replace(".",".<wbr>")
        .replace("_","_<wbr>")
        .replace("-","_<wbr>")
        .replace("/","/<wbr>")
      )).unwrap();
    };
 println!("finished");
});

"success".into()
}
