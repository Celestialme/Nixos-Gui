

lazy_static! {
pub static ref  pkgs:serde_json::Value =  serde_json::from_str(&match std::fs::read_to_string("/etc/NIX_GUI/packages.json"){
    Ok(txt) => txt,
    Err(err) => "{\"error\":\"file not found\" }".to_string()
}).unwrap();
static ref KEYS:Vec<String> = {
    let mut keys = Vec::new();
    for (key, val) in pkgs.as_object().unwrap().iter() {
        keys.push(key.to_owned());
    };
    keys
};
}
pub fn filter(value:&str,mut keys:Vec<String>) ->Vec<String>{
    
if keys.is_empty(){
   keys = KEYS.to_vec();
}

if !value.is_empty(){
keys = keys.into_iter().filter(|key| {
    key.contains(&value) || (!pkgs[key]["pname"].is_null() && pkgs[key]["pname"].as_str().unwrap().to_string().contains(&value) ) ||  ( !pkgs[key]["description"].is_null() && pkgs[key]["description"].as_str().unwrap().to_string().contains(&value))

}).collect();
};
let now = std::time::Instant::now();
keys.sort_by(|a,b|{
let by_pname = (match pkgs[b]["pname"].as_str().unwrap().to_string().starts_with(&value){true=>&1,false=>&0}).cmp(match pkgs[a]["pname"].as_str().unwrap().to_string().starts_with(&value){true=>&1,false=>&0}); // sort  by pname
let key_a = get_key_name(a,1);
let key_b = get_key_name(b,1);
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
    pkgs[b]["pname"].as_str().unwrap().to_string().chars().count().cmp(&pkgs[a]["pname"].as_str().unwrap().to_string().chars().count())
}else{
    by_pname
}


});
println!("Elapsed: {:.2?}", now.elapsed());
keys = keys.iter().map(|key| {
let mut pkg_body = pkgs[&key].as_object().unwrap().clone();
pkg_body.insert("key".to_owned(),serde_json::Value::String(key.to_owned()));
serde_json::to_string(&pkg_body).unwrap()
}).collect::<Vec<String>>();
keys.truncate(50);
keys


}
fn get_key_name(key:&str,overhead:usize) -> String{
    let mut tmp:Vec<String> = key.split(".").map(|x| x.to_string()).collect();
   tmp.drain( 0..overhead);
  
   tmp.join(".").to_owned()

}



