use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::{Read, Write};

pub struct Writer {
    path : String,
}

impl Writer {
    pub fn new() -> Writer {
        Writer {
            path : "/Users/nox/Desktop/Nox/NoxV9Database/NoxV9Database/Database/".to_string(),
        
        }
    }

    pub fn write_database(&self, mut data: String) -> std::io::Result<()> {
        //create a string. 
        let mut database : String = self.read_database();
        data.push_str("\n");
        database.push_str(&data);

        let res : std::io::Result<()> = self.write_to_file(database);

        return res;
        
    }

    pub fn set_cluster(mut self, name : String) -> Writer{
        self.path.push_str(&name); 
        if(!Path::new(&self.path).exists()) {
            //if you are not a cluster rtrun a error and tell the use to create a new cluster (CustomObject).
           let _= File::create(&self.path);
        }
        self
    }

    fn write_to_file(&self, data: String) ->  std::io::Result<()> {
        let mut file = File::create(&self.path)?;
        file.write(data.as_bytes())?;
        println!("file is written!");
        Ok(())
    
    }


    pub fn read_database(&self) -> String{
        let data : String = fs::read_to_string(&self.path).expect("can read file");  
        return data;
    }

    pub fn get_database_length(&self) -> usize {
        let data : String = self.read_database();
        let indexed : Vec<&str> = data.split("\n").collect();
        let length : usize = indexed.len(); 
        return length;
    }

    pub fn read_database_id(&self,id: i32) -> String{
        let data : String = self.read_database();
        let indexable : Vec<&str> = data.split("\n").collect();
        return indexable[id as usize].to_string();
    }

    pub fn get_data_points(&self,row: i32, col: i32) -> String{
        let data : String = self.read_database_id(row);
        let indexable : Vec<&str> = data.split(",").collect();
        return indexable[col as usize].to_string();
    }

    
}