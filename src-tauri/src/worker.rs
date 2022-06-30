

    use std::sync::{Arc,Mutex};
    use std::collections::HashMap;
lazy_static! {
pub static ref  PKGS:serde_json::Value =  serde_json::from_str(&match std::fs::read_to_string("/etc/NIX_GUI/packages.json"){
    Ok(txt) => txt,
    Err(err) => "{\"error\":\"file not found\" }".to_string()
}).unwrap();
static ref KEYS:Vec<String> = {
    let mut keys = Vec::new();
    for (key, val) in PKGS.as_object().unwrap().iter() {
        keys.push(key.to_owned());
    };
    keys
};
pub static ref CURRENT_VALUE:Mutex<String> = Mutex::new("".to_owned());
pub static ref  OPTION_LIST  :serde_json::Value =  serde_json::from_str(&match std::fs::read_to_string("/etc/NIX_GUI/options.json"){
    Ok(txt) => txt,
    Err(err) => "{\"error\":\"file not found\" }".to_string()
}).unwrap();
}
pub fn filter(value:&str,mut keys:Vec<String>) ->Vec<String>{
    
if keys.is_empty(){
   keys = KEYS.to_vec();
}

if !value.is_empty(){
keys = keys.into_iter().filter(|key| {
    key.contains(&value) || (!PKGS[key]["pname"].is_null() && PKGS[key]["pname"].as_str().unwrap().to_string().contains(&value) ) ||  ( !PKGS[key]["description"].is_null() && PKGS[key]["description"].as_str().unwrap().to_string().contains(&value))

}).collect();
};
if *CURRENT_VALUE.lock().unwrap()!= value{
    return Vec::new();
}
keys.sort_by(|a,b|{
let by_pname = (match PKGS[b]["pname"].as_str().unwrap().to_string().starts_with(&value){true=>&1,false=>&0}).cmp(match PKGS[a]["pname"].as_str().unwrap().to_string().starts_with(&value){true=>&1,false=>&0}); // sort  by pname
let key_a = get_key_name(a);
let key_b = get_key_name(b);
let by_key_name = (match key_b.contains(value){true=>&1,false=>&0}).cmp(match key_a.contains(value){true=>&1,false=>&0});
if by_key_name != std::cmp::Ordering::Equal{ 
    by_key_name
}else if key_a.contains(value) {  // if both key includes value, place first the one which starts with value
    let by_start = (match key_b.starts_with(value){true=>1,false=>0}).cmp(match key_a.starts_with(value){true=>&1,false=>&0});
    let by_length  = key_a.chars().count().cmp(&key_b.chars().count());
    let mut ord =0;
    if by_start != std::cmp::Ordering::Equal {
        by_start
    }else{
        by_length
    }
    
}else if by_pname ==std::cmp::Ordering::Equal {
    PKGS[b]["pname"].as_str().unwrap().to_string().chars().count().cmp(&PKGS[a]["pname"].as_str().unwrap().to_string().chars().count())
}else{
    by_pname
}


});
if *CURRENT_VALUE.lock().unwrap()!= value{
    return Vec::new();
}
keys = keys.iter().map(|key| {
let mut pkg_body = PKGS[&key].as_object().unwrap().clone();
pkg_body.insert("key".to_owned(),serde_json::Value::String(key.to_owned()));
serde_json::to_string(&pkg_body).unwrap()
}).collect::<Vec<String>>();
keys.truncate(50);
keys


}
fn get_key_name(key:&str) -> String{
  let re =  regex::Regex::new(r"^nixos\.").unwrap();
  re.replace_all(key,"").to_string()

}


fn filter_key(filter_key:&str){ // for submenus
let  temp:HashMap<String, serde_json::Value> = HashMap::new();
for (key, val) in PKGS.as_object().unwrap().iter() {
   if !filter_key.is_empty() && key.starts_with(filter_key) && !key.contains("<name>"){
       let temp_key;
       if key.split(".").map(|x|x.to_string()).collect::<Vec<String>>().contains(&get_key_name(filter_key)){
        temp_key = regex::Regex::new(&(filter_key.to_string()+"\\.?")).unwrap().replace_all(key,"");
       }else{
           let temp_filter_key = match regex::Regex::new(r"^.*\.").unwrap().find(key) {
               Some(x) => x.as_str(),
               None => ""
           };
       }
   }
};
}

