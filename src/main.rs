// fn main() {
//  let content =  std::fs::read_to_string("./configuration.nix").expect("configuration file not found");

//   let ast = rnix::parse("{a = b;}");

//    serde_json::to_writer(std::io::stdout(), &ast.node()).unwrap();


// }

use std::io::{BufRead, BufReader, Read};
use std::process::{Command, Stdio,Child,ChildStdin,ChildStdout};
use std::sync::{Mutex};
use std::io::Write;
use std::{thread, vec};
use rocket::serde::json::Json;
use serde::{Serialize, Deserialize};
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

#[macro_use] extern crate rocket;
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
pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Attaching CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[get("/getConfig")]
fn get_config() -> String  {

  let text =  std::fs::read_to_string("./src/configuration.nix").expect("configuration file not found");

    let ast = rnix::parse(&text);
    
    let configuration =serde_json::to_string(&ast.node()).unwrap();
    
    configuration
   
  }

#[options("/saveConfig")]
fn config_option() -> String{

"yees".into()
}
#[post("/saveConfig",format ="json",data = "<payload>")]
fn save_config(payload:Json<String>) -> String{
    std::fs::write("./src/configuration.nix", payload.to_string()).expect("could not write to configuration");
    "saved".into()
}


#[get("/repl")]
fn repl() -> String  {
   
    STDIN.lock().unwrap().write_all(b"builtins.toJSON (builtins.attrNames config.users.users)\n").unwrap();
  

 
    format!("wait for response")
    
    
  }

  #[get("/repl2")]
fn repl2() -> String  {
   
    // let mut result=[0;10000];
    // std::thread::sleep(std::time::Duration::from_millis(1000));
    // let n =STDOUT.lock().unwrap().read(&mut result).unwrap();
    // // println!("{:?}",std::str::from_utf8(&result[..n]).unwrap());
    // println!("{}",n);

  
      format!("")
  
    
    
  }

#[get("/")]
fn index() -> String  {


    
     format!("Hello, world! ")
   
  }

#[launch]
fn rocket() -> _ {
  println!("start");
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
   rocket::build().attach(CORS).mount("/", routes![index,config_option,save_config,get_config,repl,repl2])

}

