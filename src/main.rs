// fn main() {
//  let content =  std::fs::read_to_string("./configuration.nix").expect("configuration file not found");

//   let ast = rnix::parse("{a = b;}");

//    serde_json::to_writer(std::io::stdout(), &ast.node()).unwrap();


// }
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio,ChildStdin};
use std::io::Write;
use rocket::serde::json::Json;
use serde::{Serialize, Deserialize};
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
#[macro_use] extern crate rocket;
static mut STDIN:Vec<ChildStdin> = vec![];

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

#[get("/")]
fn index() -> String  {

  unsafe{

    STDIN[0].write_all(b":l <nixpkgs>\n").unwrap();
  }
    
     format!("Hello, world! ")
   
  }

#[launch]
fn rocket() -> _ {
 
   let mut child = Command::new("cat")
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()
    .expect("failed to execute child");

 let  stdin = child.stdin.take().expect("failed to get stdin");
unsafe{
  STDIN.push(stdin);
}
let  stdout = child.stdout.take().unwrap();  
let out = BufReader::new(stdout);
  
std::thread::spawn(move ||{
 out.lines().for_each(|line|{
        if line.as_ref().unwrap()==""{return}
        println!("out: {}", line.unwrap());
   }
    );
  
});


rocket::build().attach(CORS).mount("/", routes![index,config_option,save_config,get_config])

}

