// fn main() {
//  let content =  std::fs::read_to_string("./configuration.nix").expect("configuration file not found");

//   let ast = rnix::parse("{a = b;}");

//    serde_json::to_writer(std::io::stdout(), &ast.node()).unwrap();

// }
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio,ChildStdout};
use std::io::{Read,Write};
use tauri::{Manager, Window};
  

pub fn download(app:String,window:Window){
  let mut success = "true";
 let  child = Command::new("nix-env").arg("-iA").arg("nixos.".to_owned()+&app).arg("--dry-run")
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
   .stderr(Stdio::piped())
    .output()
    .expect("failed to execute child");


 let mut child2 = Command::new("nix-env").arg("-iA").arg("nixos.".to_owned()+&app).args(["--option", "sandbox", "false"])
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .expect("failed to execute child");
// let mut stdin = child.stdin.take().expect("failed to get stdin");
 println!("{:?}",child);
let  stdout = child2.stderr.take().unwrap();  
let out = BufReader::new(stdout);
std::thread::spawn(move ||{
  let output = std::str::from_utf8(&child.stderr).unwrap();
  let list: Vec<&str> = output.split('\n').collect();
  let mut start = match list.iter().position(|&r| r == "these derivations will be built:"){
           None =>  Some(0),
            Some(val) => Some(val),
  };

  let end = match list.iter().position(|&r| r.starts_with("these paths will be fetched")) {
            None => Some(if start.unwrap() >0 {start=Some(start.unwrap()+1);list.len()-1} else {0}), // if we dont have anything to build make start and end 0
            Some(val) => Some(if start.unwrap() >0 {val} else {0}),
        };

  let  build_list = list[start.unwrap()..end.unwrap()].to_vec();
  let mut build_list = build_list.iter().map(|&x| x.trim()).collect::<Vec<_>>();
  let build_length = build_list.len();
  // println!("{:?}",build_list);
  window.emit(&format!("{}-{}","progress",app.replace(".","")), std::str::from_utf8(&strip_ansi_escapes::strip(format!("[{},{}]",0,1)).unwrap()).unwrap()).unwrap();

 out.lines().for_each(|line|{
        if line.as_ref().unwrap()==""{return}
        if line.as_ref().unwrap().contains("error") || success =="false"{
            success = "false";    
            println!("success is {} for this line {}",success,line.as_ref().unwrap());
            
            return
        };
      println!("out2:      {}",line.as_ref().unwrap());
   if line.as_ref().unwrap().starts_with("building"){
        match build_list.iter().position(|&x| line.as_ref().unwrap().contains(x)) {
          None => "None",
          Some(val) =>  {build_list.remove(val);
            window.emit(&format!("{}-{}","progress",app.replace(".","")), std::str::from_utf8(&strip_ansi_escapes::strip(format!("[{},{}]",build_length-build_list.len(),build_length)).unwrap()).unwrap()).unwrap();
            "DONE"
        },
        };
   };
       
   println!("{}/{}",build_length-build_list.len(),build_length)
   }
    );
    window.emit(&format!("{}-{}","progress",app.replace(".","")), std::str::from_utf8(&strip_ansi_escapes::strip(format!("[{},{}]",1,1)).unwrap()).unwrap()).unwrap();
    window.emit(&format!("{}-{}","finish",app.replace(".","")), success).unwrap();

});



}