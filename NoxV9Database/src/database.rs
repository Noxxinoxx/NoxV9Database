// use std::collections::HashMap; Not currently used but might be in the future
// use std::io::ErrorKind::ConnectionAborted; - || -

use crate::hashing;
use crate::databasewriter;
use crate::databasewriter::Writer;

#[allow(dead_code)]
pub struct User {
    name : String, 
    email : String, 
    password : hashing::Hash,
}

/*
This struct has a lot of "dead" code since it has not been implemented yet
When the struct gets implemented simply remove the allow_dead.
 */

#[allow(dead_code)]
impl User {

    #[allow(dead_code)]
    pub fn new() -> User {
        User {
            name: "".to_string(),
            email : "".to_string(),
            password : hashing::Hash::new()
        }
    }

    #[allow(dead_code)]
    pub fn get_user_object(self) -> User {
        return self;
    }

    #[allow(dead_code)]
    pub fn set_user_object(self,name: String, email: String, password: String, register: bool) -> bool{
        let mut password_hash : hashing::Hash = hashing::Hash::new();
        let mut writer: databasewriter::Writer = databasewriter::Writer::new();
        let writer = writer.set_cluster(&"database2.csv".to_string());
        let mut on_error = false;
        if register {
            password_hash.gen_hash_type(&password);
        } else {

            let database = writer.read_database();
            println!("hej hej hej {}", database);
            let mut hash_type = "".to_string();
            for i in 0..writer.get_database_length() - 1 {
                println!("{}", writer.read_database_id(i as i32));
                println!("{}", writer.get_data_points(i as i32, 1));

                if name == writer.get_data_points(i as i32, 0) {
                    hash_type = writer.get_data_points(i as i32, 2); 
                    break;
                }
            }

            if hash_type == "".to_string() {
                on_error = true;
            } else {
                password_hash.set_hash_type(password_hash.retrieve_salt(&hash_type));    
            }

        
        }

        if !on_error {
            let hash = password_hash.password(&password);

            let mut object_builder : String = "".to_string();
            object_builder.push_str(&name);
            object_builder.push_str(",");
            object_builder.push_str(&email);
            object_builder.push_str(",");
            object_builder.push_str(&hash);

            let _ = writer.write_database(object_builder.to_string());
            return true;
        }
        println!("database is cleared");
        return false;
        
    }
}

pub fn new_custom_object(c_object: Vec<String>, cluster_name : String) {
    let mut dbwriter : databasewriter::Writer = databasewriter::Writer::new(); 
    let new_dbwriter : &Writer = dbwriter.set_cluster(&cluster_name);
    let mut db_format_builder : String = "".to_string();

    for i in 0..c_object.len() {
        db_format_builder.push_str(&c_object[i]);
        db_format_builder.push_str(","); 
    }
    let _ = new_dbwriter.write_database(db_format_builder);
    
}

/*
This function updates a cluster of your choosing, adding a new line of data.
Good to know, don't forget that it requires the data in Vec<String> form.
*/
pub fn update_database(data : Vec<String>, cluster_name : String) {
    //self.data.push(data);
    let mut dbwriter : databasewriter::Writer = databasewriter::Writer::new(); 
    let new_dbwriter : &Writer = dbwriter.set_cluster(&cluster_name);
    let mut db_format_builder : String = "".to_string();

    for i in 0..data.len() {
        db_format_builder.push_str(&data[i]); 
        db_format_builder.push_str(","); 
    }
    let _ = new_dbwriter.write_database(db_format_builder);
}

// This function is used when calling update by id, which needs \n instead of ,
pub fn update_entire_database(data : Vec<String>, cluster_name : &String) {
    let mut dbwriter : databasewriter::Writer = databasewriter::Writer::new();
    let new_dbwriter : &Writer = dbwriter.set_cluster(cluster_name);
    let mut db_format_builder : String = "".to_string();

    for i in 0..data.len() {
        db_format_builder.push_str(&data[i]);
        db_format_builder.push_str("\n");
    }
    db_format_builder.pop();
    db_format_builder.pop();
    let _ = new_dbwriter.write_database(db_format_builder);
}

/*
    This function returns the entire database in String format, use-full for checking
    Good to know, don't forget that it requires the data in Vec<String> form.
 */
pub fn get_database(cluster_name : String) -> String{
    
    let mut dbwriter : databasewriter::Writer = databasewriter::Writer::new(); 
    let new_dbwriter : &Writer = dbwriter.set_cluster(&cluster_name);

    new_dbwriter.read_database()

}

pub fn get_index_database(cluster_name : String, index : i32) -> String {

    let mut dbwriter : databasewriter::Writer = databasewriter::Writer::new(); 
    let new_dbwriter : &Writer = dbwriter.set_cluster(&cluster_name);

    new_dbwriter.read_database_id(index)

}

// This function takes index & data and then updates the data at that index with new data.
pub fn update_database_by_index(cluster_name : String, index : i32, data : Vec<String>) { // can add -> bool to see if update was successful, look at this later
    let mut dbwriter : databasewriter::Writer = databasewriter::Writer::new();
    let new_dbwriter : &Writer = dbwriter.set_cluster(&cluster_name);

    let db_info = new_dbwriter.read_database();
    let mut rows : Vec<String> = db_info.split('\n').into_iter().map(|x| x.to_string()).collect();
    let mut db_format_builder : String = "".to_string();

    for i in data {
        db_format_builder.push_str(&i);
        db_format_builder.push_str(",");
    }
    rows[index as usize] = db_format_builder;
    clear_database(&cluster_name);
    update_entire_database(rows, &cluster_name);
}

// This function wipes the database clean, use with caution
pub fn clear_database(cluster_name : &String) -> String {
    let mut dbwriter : databasewriter::Writer = databasewriter::Writer::new();
    let new_dbwriter : &Writer = dbwriter.set_cluster(&cluster_name);

    new_dbwriter.clear();

    "Cluster is cleared!".to_string()
}