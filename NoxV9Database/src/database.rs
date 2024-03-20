use serde_json::Value;
use std::fs;

pub fn testFunction(x: &str) -> &str {
    return x;
}


pub fn read_database() {
    let user_database = {
        let file_content = fs::read_to_string("./Database/database.json").expect("LogRocket : error"); 
        serde_json::from_str::<Value>(&file_content).expect("LogRocket : error in serializern");

    };

    println!("{:?}", serde_json::to_string_pretty(&user_database).expect("error on print out"));

}