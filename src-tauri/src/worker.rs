pub fn filter(value:&str){
    let mut keys = Vec::new();
   let pkgs_json =  match std::fs::read_to_string("/etc/NIX_GUI/packages.json"){
        Ok(txt) => txt,
        Err(err) => "{\"error\":\"file not found\" }".to_string()
      };

let pkgs:serde_json::Value = serde_json::from_str(&pkgs_json).unwrap();

for (key, val) in pkgs.as_object().unwrap().iter() {
    keys.push(key);
};

keys = keys.into_iter().filter(|&key| {
    key.contains(&value) || (!pkgs[key]["pname"].is_null() && pkgs[key]["pname"].as_str().unwrap().to_string().contains(&value) ) ||  ( !pkgs[key]["description"].is_null() && pkgs[key]["description"].as_str().unwrap().to_string().contains(&value))

}).collect();

keys.sort_by(|a,b|{
let by_pname = (match pkgs[a]["pname"].as_str().unwrap().to_string().starts_with(&value){true=>&1,false=>&0}).cmp(match pkgs[b]["pname"].as_str().unwrap().to_string().starts_with(&value){true=>&1,false=>&0}); // sort  by pname
let key_a = get_key_name(a);
let key_b = get_key_name(b);
let by_key_name = (match key_a.contains(value){true=>&1,false=>&0}).cmp(match key_b.contains(value){true=>&1,false=>&0});
if by_key_name != std::cmp::Ordering::Equal{ 
    by_key_name
}else if key_a.contains(value) {  // if both key includes value, place first the one which starts with value
    let by_contain = (match key_a.starts_with(value){true=>1,false=>0}).cmp(match key_b.starts_with(value){true=>&1,false=>&0});
    let by_length  = key_a.chars().count().cmp(&key_b.chars().count());
    let mut ord =0;
    if by_contain != std::cmp::Ordering::Equal {
        by_contain
    }else{
        by_length
    }
    
}else if by_pname ==std::cmp::Ordering::Equal {
    pkgs[a]["pname"].as_str().unwrap().to_string().chars().count().cmp(&pkgs[a]["pname"].as_str().unwrap().to_string().chars().count())
}else{
    by_pname
}


});


println!("{:?}",keys);
}
fn get_key_name(key:&str) -> String{
    let tmp:Vec<String> = key.split(".").map(|x| x.to_string()).collect();
    tmp.last().unwrap().to_owned()
}

