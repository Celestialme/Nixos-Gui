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


  // let mut child = Command::new("nix-env").arg("-iA").arg("nixpkgs.".to_owned()+&app)
  let mut child = Command::new("nix-env").arg("-iA").arg("-f").arg("/nixos-unstable").arg(&app).args(["--option","sandbox","false"])
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