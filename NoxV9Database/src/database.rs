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
        let writer = writer.set_cluster("database2.csv".to_string());
        let mut on_error = false;
        if(register) {
            password_hash.gen_hash_type(&password);
        }else {

            let database = writer.read_database();
            println!("hej hej hej {}", database);
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
        println!("database is cleared");
        return false;
        
    }

    

}


pub fn new_custom_object(cobject: Vec<String>, cluster_name : String) {
    let mut dbwriter : databasewriter::Writer = databasewriter::Writer::new(); 
    let mut new_dbwriter : databasewriter::Writer = dbwriter.set_cluster(cluster_name);
    let mut db_format_builder : String = "".to_string();

    for i in 0..cobject.len() {
        db_format_builder.push_str(&cobject[i]); 
        db_format_builder.push_str(","); 
    }
    new_dbwriter.write_database(db_format_builder);
    
}  
pub fn update_database(data : Vec<String>, cluster_name : String) {
    //self.data.push(data);
    let mut dbwriter : databasewriter::Writer = databasewriter::Writer::new(); 
    let mut new_dbwriter : databasewriter::Writer = dbwriter.set_cluster(cluster_name);
    let mut db_format_builder : String = "".to_string();

    for i in 0..data.len() {
        db_format_builder.push_str(&data[i]); 
        db_format_builder.push_str(","); 
    }
    new_dbwriter.write_database(db_format_builder);
}
pub fn get_database(cluster_name : String) -> String{
    
    let mut dbwriter : databasewriter::Writer = databasewriter::Writer::new(); 
    let mut new_dbwriter : databasewriter::Writer = dbwriter.set_cluster(cluster_name);

    new_dbwriter.read_database()

}

pub fn get_index_database(cluster_name : String, index : i32) -> String {

    let mut dbwriter : databasewriter::Writer = databasewriter::Writer::new(); 
    let mut new_dbwriter : databasewriter::Writer = dbwriter.set_cluster(cluster_name);

    new_dbwriter.read_database_id(index)

}
//save you for a rainy day.
pub fn update_database_by_index(cluster_name : String, index : i32) {



}


pub fn clear_database(cluster_name : String) -> String {
    let mut dbwriter : databasewriter::Writer = databasewriter::Writer::new(); 
    let mut new_dbwriter : databasewriter::Writer = dbwriter.set_cluster(cluster_name);

    new_dbwriter.clear();

    "Cluster is cleared!".to_string()
}