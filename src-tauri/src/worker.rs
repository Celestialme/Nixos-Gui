
    
     use tauri::Window;
    use std::sync::{Arc,Mutex};
    use std::collections::HashMap;
    use serde::{Serialize, Deserialize};
lazy_static! {
pub static ref  PKGS:serde_json::Value =  serde_json::from_str(&match std::fs::read_to_string("/etc/NIX_GUI/packages.json"){
    Ok(txt) => txt,
    Err(err) => "{\"error\":\"file not found\" }".to_string()
}).expect("package.json is corrupted, please delete it from /etc/NIX_GUI/packages.json");
static ref PKG_KEYS:Vec<String> = {
    let mut keys = Vec::new();
    for (key, val) in PKGS.as_object().unwrap().iter() {
        keys.push(key.to_owned());
    };
    keys
};
pub static ref CURRENT_VALUE:Mutex<String> = Mutex::new("".to_owned());
pub static ref  OPTION_LIST: serde_json::Map<String, serde_json::Value> =  { 
    let tmp:serde_json::Value = serde_json::from_str(&match std::fs::read_to_string("/etc/NIX_GUI/options.json"){
    Ok(txt) => txt,
    Err(err) => "{\"error\":\"file not found\" }".to_string()
}).unwrap();
tmp.as_object().unwrap().clone()
};
static ref OPTION_KEYS:Vec<String> = {
    let mut keys = Vec::new();
    for (key, val) in OPTION_LIST.iter() {
        keys.push(key.to_owned());
    };
    keys
};
}



#[derive(Serialize,Deserialize)]
struct Resp {
    Type:String,
    Value:HashMap<String, serde_json::Value>
}
#[derive(Serialize,Deserialize)]
struct Resp2 {
    Type:String,
    Value:serde_json::Value
}
pub fn filter(value:&str,mut keys:Vec<String>) ->Vec<String>{
    println!("{:?}",keys);
    let re =  regex::Regex::new(r"^nixos\.").unwrap();
if keys.is_empty(){
   keys = PKG_KEYS.to_vec();
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
let key_a = get_key_name(a,&re);
let key_b = get_key_name(b,&re);
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
fn get_key_name(key:&str,re:&regex::Regex) -> String{
  re.replace(key,"").to_string()

}


fn get_dict_key_name(key:&str)->String{
    let mut i=0;
        for _i in (0..key.chars().count()).rev(){
            
            if key.chars().nth(_i).unwrap() =='.'{
                i= _i+1;
                break
            }
        }
      
       let _key = key.chars().rev().collect::<String>();
       let end =  if (key.chars().count()-i) == 0 {key.chars().count()} else {key.chars().count()-i};
               (&_key[..end]).to_owned().chars().rev().collect::<String>()
        
    }







pub fn filter_dict(window:Window,filter_key:&str,repl_command:&dyn Fn(String) -> String ){ // for submenus
let re =regex::Regex::new(&(filter_key.to_string()+r"\.?")).unwrap();
let re2 = regex::Regex::new(r"^.*\.").unwrap();
let mut  temp:HashMap<String, serde_json::Value> = HashMap::new();
for (key, val) in OPTION_LIST.iter() {
    if *CURRENT_VALUE.lock().unwrap()!= filter_key {break};
   if !filter_key.is_empty() && key.starts_with(filter_key) && !key.contains("<name>"){
       let temp_key;
       if key.split(".").map(|x|x.to_string()).collect::<Vec<String>>().contains(&get_dict_key_name(filter_key)){ // when full section name is written
        temp_key = re.replace(key,"").to_string();
        
       }else{
           let temp_filter_key = match re2.find(filter_key) {
               Some(x) => x.as_str(),
               None => ""
           };
           temp_key = regex::Regex::new(&(temp_filter_key.to_string()+r"\.?")).unwrap().replace(key,"").to_string();
           
       }


       if !temp_key.is_empty() {
           temp.insert(temp_key, OPTION_LIST[key].clone());
       }
   } else if !filter_key.is_empty() && key.starts_with(filter_key) && key.replacen(filter_key,"",1).starts_with("<name>"){
    // postMessage({type:'filterDict-repl',value:dict[key]})
    
    let mut  filter_key = filter_key.to_string();
    filter_key.pop();
   
    let payload = format!("builtins.toJSON (builtins.attrNames config.{})",filter_key);
    let keys:Vec<serde_json::Value> = serde_json::from_str(&repl_command(payload).replace("\"","").replace(r"\","\"")).unwrap();
   
    let mut  temp:HashMap<String, serde_json::Value> = HashMap::new();
   
    for _key in keys {
        let _str = format!("<{}>",_key.as_str().unwrap().to_string());
        temp.insert(_str, OPTION_LIST[key].clone());
       
    };
    window.emit("filterDict",temp);
 
    return
   }else if filter_key.is_empty(){
    temp.insert(key.to_string(), OPTION_LIST[key].clone());
   }
};
window.emit("filterDict",temp);

}


pub fn filter_options(window:Window,mut value:String){
let _value = value.to_owned();
let re = regex::Regex::new(r"<.*>").unwrap();
value = regex::Regex::new(r"\.$").unwrap().replace(&value,"").to_string();
let mut  filtered_key:Vec<String> = OPTION_KEYS.iter().filter(|key|key.contains(
    &re.replace_all(&value,"<name>").to_string()

)).map(|x|x.to_string()).collect();


let _match = match re.find(&value){
    Some(x)=>x.as_str(),
    None =>"<name>"
};

if *CURRENT_VALUE.lock().unwrap()!= _value {return};
filtered_key.sort_by(|a,b|{
    let temp = (match b.starts_with(&value){true=>1,false=>0}).cmp(&match a.starts_with(&value){true=>1,false=>0});
    if temp == std::cmp::Ordering::Equal{
        return a.chars().count().cmp(&b.chars().count());
    }else{
        return temp;
    };
});

if *CURRENT_VALUE.lock().unwrap()!= _value {return};

let mut filtered_option = filtered_key.iter().map(|key| {
    
    let mut option_body = OPTION_LIST[key].as_object().unwrap().clone();
    option_body.insert("key".to_owned(),serde_json::Value::String( re.replace_all(&key,_match).to_string()
));
    serde_json::to_string(&option_body).unwrap()
    }).collect::<Vec<String>>();
    filtered_option.truncate(50);
    
    
window.emit("filterOptions",filtered_option);

}