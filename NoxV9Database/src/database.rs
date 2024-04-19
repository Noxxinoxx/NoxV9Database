use std::collections::HashMap;

use crate::hashing;
use crate::databasewriter;
pub struct User {
    name : String, 
    email : String, 
    password : hashing::Hash,
}

impl User {

    pub fn new() -> User {
        User {
            name: "".to_string(),
            email : "".to_string(),
            password : hashing::Hash::new()
        }
    }    

    pub fn get_user_object(self) -> User {
        return self;
    }

    pub fn set_user_object(self,name: String, email: String, password: String, register: bool) -> bool{
        let mut password_hash : hashing::Hash = hashing::Hash::new();
        let writer : databasewriter::Writer = databasewriter::Writer::new();
        let mut on_error = false;
        if(register) {
            password_hash.gen_hash_type(&password);
        }else {

            let database = writer.read_database();
            let mut hash_type = "".to_string();
            for i in 0..writer.get_database_length() - 1 {
                println!("{}", writer.read_database_id(i as i32));
                println!("{}", writer.get_data_points(i as i32, 1));

                if(name == writer.get_data_points(i as i32, 0)) {
                    hash_type = writer.get_data_points(i as i32, 2); 
                    break;
                }
            }

            if(hash_type == "".to_string()) {
                on_error = true;
            }else {
                password_hash.set_hash_type(password_hash.retrieve_salt(&hash_type));    
            }

        
        }

        if(!on_error) {
            let hash = password_hash.password(&password);

            let mut object_builder : String = "".to_string();
            object_builder.push_str(&name);
            object_builder.push_str(",");
            object_builder.push_str(&email);
            object_builder.push_str(",");
            object_builder.push_str(&hash);

            writer.write_database(object_builder.to_string());
            return true;
        }

        return false;
        
    }

    

}

//name,yuotube, time, date, bill, email, password.
//car, model, name, company.
struct CustomObject {
    data: Vec<HashMap<String, String>>
}



impl CustomObject {
    
    


}