pub fn filter(value:&str){
    let mut keys = Vec::new();
   let pkgs_str =  match std::fs::read_to_string("/etc/NIX_GUI/pkgs.json"){
        Ok(txt) => txt,
        Err(err) => "{\"error\":\"file not found\" }".to_string()
      };

let pkgs:serde_json::Value = serde_json::from_str(&pkgs_str).unwrap();

for (key, val) in pkgs.as_object().unwrap().iter() {
    keys.push(key);
};

keys = keys.iter().filter(|&key| {

    key.contains(&value) || pkgs[key]["pname"].as_str().unwrap().to_string().contains(&value) ||  pkgs[key]["description"].as_str().unwrap().to_string().contains(&value)

}).collect();
// key.includes(value) || packages[key]?.pname.includes(value) || packages[key]?.description.includes(value)



}
fn get_key_name(key:&str){
    let tmp:Vec<String> = key.split(".").map(|x| x.to_string()).collect();
    println!("{:?}",tmp.last().unwrap());
}

