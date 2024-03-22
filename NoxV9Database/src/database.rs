use serde_json::Value;
use std::fs;
use serde_json::Serializer; 
use serde_json::Deserializer;
#[derive(Serialize, Deserialize)]
struct User {
    name : String, 
    email : String, 
    password : String,
}

pub fn read_database() {
    let user_database = {
        let file_content = fs::read_to_string("./Database/database.json").expect("LogRocket : error"); 
        

        let json = serde_json::from_str::<Value>(&file_content).expect("LogRocket : error in serializern");

        let mut data : Vec<User> = Vec::new();

        if let serde_json::Value::Array(items) = json {
            for item in items {
                println!("{}", item);

                let p : User = 
            }
        }
        
        

        

    };

    println!("{:?}", &user_database);

}